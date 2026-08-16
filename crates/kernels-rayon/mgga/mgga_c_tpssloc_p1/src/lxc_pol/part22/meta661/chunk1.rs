//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2206/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2206(t16787: f64, t2563: f64, t16791: f64, t9546: f64, t2586: f64, t41146: f64, t59162: f64, t59135: f64, t9523: f64, t5624: f64, t9993: f64, t5628: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t59216 = t2563 * t16787;
    let t59218 = t9546 * t16791;
    let t59221 = t2586 * t41146 * t59162;
    let t59224 = t2586 * t9523 * t59135;
    let t59251 = t9993 * t5624;
    let t59255 = t9993 * t5628;
    (t59216, t59218, t59221, t59224, t59251, t59255)
}
