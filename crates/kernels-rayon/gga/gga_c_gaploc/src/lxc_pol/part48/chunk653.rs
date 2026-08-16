//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 653/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk653(t11656: f64, t123: f64, t734: f64, t296: f64, t3614: f64, t11638: f64, t11641: f64, t11644: f64, t11647: f64, t11650: f64, t11653: f64, t1841: f64, t1897: f64, t2508: f64, t3622: f64, t3627: f64, t3631: f64, t650: f64) -> (f64, f64, f64, f64) {
    let t11657 = t11656 * t123;
    let t11658 = t11657 * t734;
    let t11661 = t296 * t3614;
    let t11662 = t11661 * t123;
    let t11663 = t11662 * t734;
    let t11672 = 0.23071578690426672851e-1_f64 * t1897 * t11638 - 0.46143157380853345702e-1_f64 * t2508 * t11641 + 0.76905262301422242837e-2_f64 * t2508 * t11644 - 0.76905262301422242837e-2_f64 * t1897 * t11647 + 0.15381052460284448567e-1_f64 * t2508 * t11650 + 0.76905262301422242837e-2_f64 * t2508 * t11653 - 0.85450291446024714263e-3_f64 * t1841 * t11658 - 0.85450291446024714263e-3_f64 * t1841 * t11663 + 0.20508069947045931423e-1_f64 * t650 * t3627 - 0.10254034973522965712e-1_f64 * t650 * t3631 - 0.30762104920568897135e-1_f64 * t650 * t3622;
    (t11657, t11661, t11662, t11672)
}
