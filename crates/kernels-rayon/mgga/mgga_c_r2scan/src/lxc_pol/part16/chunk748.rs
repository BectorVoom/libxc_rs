//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 748/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk748(t2116: f64, t6161: f64, t6327: f64, t6329: f64, t1266: f64, t277: f64) -> (f64, f64) {
    let t6331 = t2116 * t6161;
    let t6333 = 0.25705033881751801528e-4_f64 * t6327 * t6329 * t6331;
    let t6343 = t1266 * t277;
    (t6333, t6343)
}
