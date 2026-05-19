"""One-shot diag driver: emit_per_functional(mgga_c_tpssloc) → /tmp temp dir,
with LIBXC_RS_CSE_DIAG=1 set so per_functional._cse_chunk_part prints
CSE-REJECT lines when it returns None. Writes nothing under crates/kernels/.
"""
import os
import sys
from pathlib import Path

os.environ["LIBXC_RS_CSE_DIAG"] = "1"

REPO_ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(REPO_ROOT / "tools"))

from translate_v2 import emit  # noqa: E402
import translate_mgga  # noqa: E402

TMPDIR = Path("/tmp/tpssloc-emit/crates/kernels")
TMPDIR.mkdir(parents=True, exist_ok=True)
emit.set_kernels_root(TMPDIR)

c_file = str(REPO_ROOT / "libxc-master/src/maple2c/mgga_exc/mgga_c_tpssloc.c")
mods = translate_mgga.emit_per_functional(c_file, "mgga_c_tpssloc",
                                          family="mgga", is_vxc_only=False)
print(f"\nemit_per_functional returned: {mods}")
