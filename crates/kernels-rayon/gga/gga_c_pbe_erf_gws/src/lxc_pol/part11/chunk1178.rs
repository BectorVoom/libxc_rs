//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1178/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1178(t47566: f64, t47567: f64, t47568: f64, t47570: f64, t47574: f64, t47576: f64, t47578: f64, t47580: f64, t47582: f64, t47584: f64, t47586: f64, t26242: f64, t47587: f64, t47616: f64, t47617: f64, t47618: f64, t47622: f64, t47626: f64, t47628: f64, t47629: f64, t47630: f64, t47631: f64, t47632: f64) -> (f64, f64) {
    let t48632 = t47566 + t47567 - t47568 + t47570 - t47574 + t47576 + t47578 + t47580 + t47582 + t47584 + t47586;
    let t48634 = -t47587 + t47616 + 0.12985249634837812052e1_f64 * t26242 - t47617 - t47618 + t47622 - t47626 - t47628 + t47629 + t47630 - t47631 - t47632;
    (t48632, t48634)
}
