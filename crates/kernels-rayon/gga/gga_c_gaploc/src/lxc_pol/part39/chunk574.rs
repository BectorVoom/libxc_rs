//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 574/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk574(t1897: f64, t2508: f64, t9592: f64, t9597: f64, t9600: f64, t9605: f64, t9608: f64, t9611: f64, t9614: f64, t9618: f64, t9620: f64, t9622: f64, t9661: f64, t9718: f64, t9763: f64) -> f64 {
    let t9765 = 0.30762104920568897134e-1_f64 * t2508 * t9592 + 0.76905262301422242837e-2_f64 * t1897 * t9597 - 0.46143157380853345702e-1_f64 * t2508 * t9600 - 0.15381052460284448567e-1_f64 * t1897 * t9605 - 0.76905262301422242837e-2_f64 * t1897 * t9608 + 0.76905262301422242837e-2_f64 * t2508 * t9611 + 0.23071578690426672851e-1_f64 * t1897 * t9614 + t9618 - t9620 - t9622 + t9661 + t9718 + t9763;
    t9765
}
