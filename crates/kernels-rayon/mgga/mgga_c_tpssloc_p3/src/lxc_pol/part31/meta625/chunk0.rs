//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1882/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1882(t19951: f64, t22833: f64, t19972: f64, t19976: f64, t5259: f64, t91100: f64, t26308: f64, t5234: f64, t5252: f64, t6396: f64, t80820: f64, t19962: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t97208 = t22833 * t19951;
    let t97210 = t22833 * t19972;
    let t97212 = t22833 * t19976;
    let t97214 = t91100 * t5259;
    let t97217 = t5234 * t26308 * t5252;
    let t97219 = t80820 * t6396;
    let t97221 = t22833 * t19962;
    (t97208, t97210, t97212, t97214, t97217, t97219, t97221)
}
