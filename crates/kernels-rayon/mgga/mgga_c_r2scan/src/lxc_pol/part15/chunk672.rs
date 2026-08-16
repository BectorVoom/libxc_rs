//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 672/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk672(t48: f64, t4902: f64, t35: f64, t40: f64, t51: f64, t53: f64, t139: f64, t141: f64, t1524: f64, t378: f64, t735: f64, t1385: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4904 = 1.0_f64 / t48 / t4902;
    let t4911 = t35 * t40;
    let t4918 = t51 * t51;
    let t4920 = 1.0_f64 / t53 / t4918;
    let t4938 = 1.0_f64 / t139;
    let t4948 = 1.0_f64 / t141;
    let t4962 = t378 * t1524;
    let t4963 = t735 * t4962;
    let t4964 = 0.32530743900905219526e-1_f64 * t4963;
    let t4965 = t378 * t1385;
    (t4904, t4911, t4920, t4938, t4948, t4964, t4965)
}
