//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2314/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2314(t1222: f64, t27589: f64, t1184: f64, t1409: f64, t24682: f64, t460: f64, t461: f64, t1009: f64, t7324: f64, t24722: f64, t15548: f64, t24733: f64) -> (f64, f64, f64, f64, f64) {
    let t95410 = t27589 * t1222 / 216.0_f64;
    let t95413 = t1409 * t1184;
    let t95415 = t24682 * t95413 * t460;
    let t95420 = t1409 * t461;
    let t95422 = t7324 * t95420 * t1009;
    let t95424 = 0.20186378047070195428e-3_f64 * t95422 * t24722;
    let t95435 = t24733 * t15548 / 1152.0_f64;
    (t95410, t95413, t95415, t95424, t95435)
}
