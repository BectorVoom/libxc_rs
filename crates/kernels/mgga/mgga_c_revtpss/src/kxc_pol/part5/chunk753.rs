//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 753/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk753<F: Float>(t1248: F, t471: F, t5332: F, t3720: F, t1222: F, t1235: F, t1238: F, t1252: F, t1261: F, t1791: F, t3637: F, t3667: F, t3711: F, t5293: F, t5299: F, t5304: F, t5309: F, t5313: F, t5320: F, t5323: F, t5327: F, t5331: F) -> (F, F, F, F) {
    let t5333 = t1248 * t471;
    let t5334 = t5332 * t5333;
    let t5335 = t3720 * t5334;
    let t5338 = -0.11433071498151929859e-2 * t5293 * t1252 + 0.14291339372689912324e-3 * t3711 * t5299 + 0.23818898954483187207e-3 * t1261 * t5304 - 0.95275595817932748827e-4 * t3637 - t1222 * t5309 / 144.0 + t1222 * t5313 / 216.0 - 0.21437009059034868486e-3 * t3667 * t1791 - 0.21437009059034868486e-3 * t1235 * t5320 + 0.11433071498151929859e-2 * t5323 * t1238 - 0.21437009059034868486e-3 * t5327 * t1238 - 0.21437009059034868486e-3 * t5331 * t5335;
    (t5333, t5334, t5335, t5338)
}
