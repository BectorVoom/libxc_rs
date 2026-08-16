//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2242/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2242(t12050: f64, t3151: f64, t357: f64, t15907: f64, t3133: f64, t3302: f64, t4893: f64, t3059: f64, t4975: f64, t4781: f64, t12132: f64, t1647: f64, t3316: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t16568 = t12050 * t3151 * t357;
    let t16569 = t15907 * t16568;
    let t16573 = t3302 * t3133 * t357;
    let t16574 = t4893 * t16573;
    let t16577 = t4975 * t3059;
    let t16578 = t4781 * t16577;
    let t16581 = t4893 * t12132;
    let t16584 = t1647 * t3316;
    (t16568, t16569, t16573, t16574, t16577, t16578, t16581, t16584)
}
