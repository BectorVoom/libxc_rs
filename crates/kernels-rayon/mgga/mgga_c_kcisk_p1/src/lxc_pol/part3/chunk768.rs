//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 768/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk768(t11798: f64, t1937: f64, t11450: f64, t719: f64, t735: f64, t1935: f64, t640: f64, t11154: f64, t746: f64, t741: f64, t10436: f64, t7311: f64, sigma2: f64) -> (f64, f64, f64, f64) {
    let t11799 = t11798 * sigma2;
    let t11800 = t11799 * t1937;
    let t11802 = t719 * t11450;
    let t11803 = t735 * t11802;
    let t11804 = t1935 * t11803;
    let t11807 = 1.0_f64 / t719 / t640;
    let t11808 = t11807 * t11154;
    let t11809 = t746 * t11808;
    let t11810 = t741 * t11809;
    let t11812 = t7311 * t10436;
    (t11800, t11804, t11810, t11812)
}
