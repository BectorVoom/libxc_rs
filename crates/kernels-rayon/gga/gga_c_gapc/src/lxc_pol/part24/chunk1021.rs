//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1021/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1021(t2580: f64, t7943: f64, t147: f64, t786: f64, t3412: f64, t8133: f64, t4978: f64, t7073: f64, t2188: f64, t314: f64, t959: f64, t7591: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16798 = t2580 * t7943;
    let t16826 = t147 * t786;
    let t17713 = t3412 * t8133;
    let t17760 = t7073 * t4978;
    let t17819 = t2188 * t959 * t314;
    let t17874 = t7591 * t314;
    (t16798, t16826, t17713, t17760, t17819, t17874)
}
