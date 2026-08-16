//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2843/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2843(t11986: f64, t828: f64, t3091: f64, t3096: f64, t12097: f64, t3090: f64, t11273: f64, t12012: f64, t11631: f64, t3133: f64, t1086: f64, t11223: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43240 = t828 * t11986;
    let t43242 = t3091 * t43240 * t3096;
    let t43244 = t12097 * t3090;
    let t43268 = t11273 * t12012;
    let t43279 = t11631 * t3133;
    let t43285 = t11223 * t1086 * t3090;
    (t43240, t43242, t43244, t43268, t43279, t43285)
}
