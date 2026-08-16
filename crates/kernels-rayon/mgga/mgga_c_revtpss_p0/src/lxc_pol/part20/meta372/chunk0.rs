//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1352/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1352(t10867: f64, t860: f64, t2722: f64, t2723: f64, t10069: f64, t10929: f64, t138: f64, t785: f64, t9302: f64, t2786: f64, t10073: f64, t10920: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40258 = t10867 * t860;
    let t40262 = t2722 * t2722;
    let t40263 = t40262 * t2723;
    let t40267 = t10069 * t10929;
    let t40270 = t138 * t9302 * t785;
    let t40271 = t40270 * t2786;
    let t40273 = t10073 * t10920;
    (t40258, t40262, t40263, t40267, t40270, t40271, t40273)
}
