//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1279/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1279(t11243: f64, t18790: f64, t11261: f64, t2192: f64, t11184: f64, t18480: f64, t836: f64, t3046: f64, t9771: f64, t3747: f64, t7966: f64, t3041: f64, t9798: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31196 = 0.96491876992155210402e2_f64 * t18790 * t11243;
    let t31198 = 1.0_f64 * t2192 * t11261;
    let t31204 = t18480 * t11184 * t836;
    let t31206 = t9771 * t3046;
    let t31208 = t7966 * t3747;
    let t31210 = t3041 * t9798;
    (t31196, t31198, t31204, t31206, t31208, t31210)
}
