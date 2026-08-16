//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 569/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk569(t739: f64, t9688: f64, t738: f64, t169: f64, t299: f64, t706: f64, t3266: f64, t702: f64, t2532: f64, t954: f64, t1897: f64, t2508: f64, t270: f64, t3212: f64, t3237: f64, t3244: f64, t3252: f64, t3256: f64, t650: f64, t681: f64) -> (f64, f64) {
    let t9689 = t739 * t9688;
    let t9690 = t738 * t9689;
    let t9698 = t9688 * t169 * t299;
    let t9699 = t706 * t9698;
    let t9712 = t3266 * t702;
    let t9715 = t954 * t2532;
    let t9718 = -0.76905262301422242837e-2_f64 * t270 * t9690 + 0.76905262301422242837e-2_f64 * t681 * t3212 + 0.76905262301422242837e-2_f64 * t681 * t3237 + 0.76905262301422242837e-2_f64 * t270 * t9699 - 0.10254034973522965712e-1_f64 * t650 * t3256 + 0.20508069947045931423e-1_f64 * t650 * t3252 - 0.30762104920568897135e-1_f64 * t650 * t3244 + 0.10254034973522965712e-1_f64 * t650 * t3212 + 0.10254034973522965712e-1_f64 * t650 * t3237 - 0.76905262301422242837e-2_f64 * t1897 * t9712 + 0.15381052460284448567e-1_f64 * t2508 * t9715;
    (t9689, t9718)
}
