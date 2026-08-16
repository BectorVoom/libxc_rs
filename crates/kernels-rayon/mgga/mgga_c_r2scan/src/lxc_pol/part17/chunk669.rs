//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 669/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk669(t51: f64, t53: f64, t139: f64, t141: f64, t1524: f64, t378: f64, t735: f64, t1385: f64, t1527: f64, t1751: f64, t1411: f64, t1398: f64, t468: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4918 = t51 * t51;
    let t4920 = 1.0_f64 / t53 / t4918;
    let t4938 = 1.0_f64 / t139;
    let t4948 = 1.0_f64 / t141;
    let t4962 = t378 * t1524;
    let t4963 = t735 * t4962;
    let t4964 = 0.32530743900905219526e-1_f64 * t4963;
    let t4965 = t378 * t1385;
    let t4966 = t735 * t4965;
    let t4967 = 0.48159733137676571078e0_f64 * t4966;
    let t4968 = t1751 * t1527;
    let t4970 = t378 * t1411;
    let t4971 = t735 * t4970;
    let t4972 = 0.16265371950452609763e-1_f64 * t4971;
    let t4973 = t1398 * t468;
    (t4920, t4938, t4948, t4964, t4967, t4968, t4972, t4973)
}
