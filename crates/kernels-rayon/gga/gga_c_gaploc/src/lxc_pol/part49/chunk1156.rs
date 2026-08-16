//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1156/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1156(t13942: f64, t650: f64, t270: f64, t47420: f64, t738: f64, t681: f64, t43040: f64, t43043: f64, t43046: f64, t43049: f64, t43051: f64, t43053: f64, t43054: f64, t47629: f64) -> f64 {
    let t47631 = 0.10254034973522965712e-1_f64 * t650 * t13942;
    let t47634 = 0.76905262301422242837e-2_f64 * t270 * t738 * t47420;
    let t47636 = 0.76905262301422242837e-2_f64 * t681 * t13942;
    let t47639 = -t43040 - t47629 + t47631 - t47634 + t47636 + t43043 + 0.25635087433807414279e-2_f64 * t43046 - t43049 - 0.23071578690426672851e-1_f64 * t43051 - t43053 + t43054;
    t47639
}
