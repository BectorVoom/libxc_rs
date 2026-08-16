//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 943/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk943(t225: f64, t33259: f64, t22704: f64, t33249: f64, t81326: f64, t33297: f64, t6883: f64, t22674: f64, t33296: f64, t6897: f64, t22751: f64, t33307: f64) -> (f64, f64, f64, f64, f64) {
    let t122172 = t33259 * t225;
    let t122178 = t22704 * t81326 * t33249;
    let t122210 = t6883 * t33297;
    let t122247 = t6897 * t22674 * t33296;
    let t122251 = t22751 * t33307;
    (t122172, t122178, t122210, t122247, t122251)
}
