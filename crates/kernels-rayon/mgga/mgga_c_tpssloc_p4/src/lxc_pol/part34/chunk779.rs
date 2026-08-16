//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 779/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk779(t15501: f64, t3503: f64, t3500: f64, t1210: f64, t1742: f64, t478: f64, t3068: f64, t1244: f64, t1734: f64, t3508: f64, t1744: f64, t3540: f64) -> (f64, f64, f64, f64, f64) {
    let t15502 = t3503 * t15501;
    let t15503 = t3500 * t15502;
    let t15506 = t1210 * t15501;
    let t15507 = t3500 * t15506;
    let t15567 = t478 * t1742;
    let t15568 = t15567 * t3068;
    let t15569 = t1244 * t15568;
    let t15659 = t1734 * t3508;
    let t15717 = t1744 * t3540;
    (t15503, t15507, t15569, t15659, t15717)
}
