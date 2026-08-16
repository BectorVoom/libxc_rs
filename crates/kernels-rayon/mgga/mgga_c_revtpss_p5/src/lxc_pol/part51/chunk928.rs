//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 928/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk928(t385: f64, t999: f64, t247: f64, t3116: f64, t3140: f64, t8507: f64, t1078: f64, t1982: f64, t25669: f64, t3268: f64, t8513: f64, t3143: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31921 = t385 * t999;
    let t31923 = t247 * t3116 * t31921;
    let t31926 = t8507 * t3140;
    let t31927 = t31926 * t1078;
    let t31928 = t1982 * t31927;
    let t31934 = t8513 * t25669 * t3268;
    let t31935 = t3143 * t8507;
    (t31921, t31923, t31926, t31927, t31928, t31934, t31935)
}
