//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1178/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1178<F: Float>(t47566: F, t47567: F, t47568: F, t47570: F, t47574: F, t47576: F, t47578: F, t47580: F, t47582: F, t47584: F, t47586: F, t26242: F, t47587: F, t47616: F, t47617: F, t47618: F, t47622: F, t47626: F, t47628: F, t47629: F, t47630: F, t47631: F, t47632: F) -> (F, F) {
    let t48632 = t47566 + t47567 - t47568 + t47570 - t47574 + t47576 + t47578 + t47580 + t47582 + t47584 + t47586;
    let t48634 = -t47587 + t47616 + F::cast_from(0.12985249634837812052e1_f64) * t26242 - t47617 - t47618 + t47622 - t47626 - t47628 + t47629 + t47630 - t47631 - t47632;
    (t48632, t48634)
}
