//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 541/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk541(t123: f64, t7284: f64, t2563: f64, t9647: f64, t5539: f64, t7292: f64, t1841: f64, t3244: f64, t3252: f64, t3256: f64, t681: f64, t9627: f64, t9629: f64, t9632: f64, t9635: f64, t9638: f64, t9643: f64) -> (f64, f64, f64, f64) {
    let t9648 = t7284 * t123;
    let t9649 = t9648 * t2563;
    let t9651 = 0.1922631557535556071e-2_f64 * t9647 * t9649;
    let t9652 = t5539 * t7292;
    let t9654 = 0.1281754371690370714e-2_f64 * t9647 * t9652;
    let t9661 = -t9627 + t9629 + t9632 - t9635 - 0.85450291446024714263e-3_f64 * t1841 * t9638 - 0.85450291446024714263e-3_f64 * t1841 * t9643 - t9651 + t9654 - 0.23071578690426672851e-1_f64 * t681 * t3244 + 0.15381052460284448567e-1_f64 * t681 * t3252 - 0.76905262301422242837e-2_f64 * t681 * t3256;
    (t9648, t9651, t9654, t9661)
}
