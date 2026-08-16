//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 668/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk668(t12166: f64, t738: f64, t12255: f64, t740: f64, t3732: f64, t702: f64, t10631: f64, t10634: f64, t10638: f64, t10642: f64, t1897: f64, t2508: f64, t270: f64, t3727: f64, t681: f64, t9618: f64, t9620: f64, t9622: f64, t9627: f64, t9629: f64, t9632: f64) -> f64 {
    let t12281 = t738 * t12166;
    let t12284 = t12255 * t740;
    let t12287 = t3732 * t702;
    let t12290 = -0.76905262301422242837e-2_f64 * t681 * t3727 - 0.76905262301422242837e-2_f64 * t270 * t12281 + t9618 - t9620 - t9622 - t9627 + t9629 + t9632 - t10631 + t10634 - t10638 - 0.23071578690426672851e-1_f64 * t2508 * t12284 - 0.76905262301422242837e-2_f64 * t1897 * t12287 - t10642;
    t12290
}
