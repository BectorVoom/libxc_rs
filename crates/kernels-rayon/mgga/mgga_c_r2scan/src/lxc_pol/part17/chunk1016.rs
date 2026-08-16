//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1016/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1016(t12725: f64, t12567: f64, t3469: f64, t12422: f64, t11189: f64, t12574: f64, t3275: f64, t1039: f64, t3787: f64, t12098: f64, t3582: f64, t12414: f64, t3465: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12726 = 3.0_f64 / 4.0_f64 * t12725;
    let t12727 = t12567 * t3469;
    let t12728 = t12727 / 4.0_f64;
    let t12729 = t12422 * t3469;
    let t12730 = t12729 / 4.0_f64;
    let t12732 = t3275 * t11189 * t12574;
    let t12733 = 45.0_f64 / 64.0_f64 * t12732;
    let t12734 = t1039 * t3787;
    let t12735 = 2.0_f64 * t12734;
    let t12737 = t3275 * t12098 * t3582;
    let t12738 = 5.0_f64 / 8.0_f64 * t12737;
    let t12739 = t3465 * t12414;
    (t12726, t12728, t12730, t12733, t12735, t12738, t12739)
}
