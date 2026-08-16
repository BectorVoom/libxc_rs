//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1930/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1930(t26308: f64, t3777: f64, t5252: f64, t16257: f64, t26309: f64, t5293: f64, t80820: f64, t5259: f64, t80816: f64, t16244: f64, t22833: f64, t5303: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t91116 = t3777 * t26308 * t5252;
    let t91118 = t26309 * t16257;
    let t91120 = t80820 * t5293;
    let t91122 = t80816 * t5259;
    let t91124 = t22833 * t16244;
    let t91126 = t80816 * t5303;
    (t91116, t91118, t91120, t91122, t91124, t91126)
}
