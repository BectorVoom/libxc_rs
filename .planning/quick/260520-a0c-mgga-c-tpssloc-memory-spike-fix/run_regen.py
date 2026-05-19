"""Regen mgga_c_tpssloc into the real crates/kernels path with the raised
wrapper cap. Requires LIBXC_RS_ACCEPT_OVERSIZED_WRAPPER=1 in the environment.
"""
import os
import sys
from pathlib import Path

assert os.environ.get("LIBXC_RS_ACCEPT_OVERSIZED_WRAPPER"), \
    "Set LIBXC_RS_ACCEPT_OVERSIZED_WRAPPER=1 before running this script"

REPO_ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(REPO_ROOT / "tools"))

import translate_mgga  # noqa: E402

c_file = str(REPO_ROOT / "libxc-master/src/maple2c/mgga_exc/mgga_c_tpssloc.c")
mods = translate_mgga.emit_per_functional(c_file, "mgga_c_tpssloc",
                                          family="mgga", is_vxc_only=False)
print(f"\nemit_per_functional returned {len(mods)} output modules: {mods}")
