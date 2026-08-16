//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 559/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk559(t2963: f64, t738: f64, t1025: f64, t1030: f64, t1841: f64, t1897: f64, t2508: f64, t2545: f64, t2550: f64, t2556: f64, t2560: f64, t2565: f64, t270: f64, t2909: f64, t2912: f64, t2928: f64, t2933: f64, t2937: f64, t2951: f64, t2955: f64, t2960: f64, t650: f64, t681: f64) -> (f64, f64) {
    let t2964 = t738 * t2963;
    let t2967 = 0.10254034973522965712e-1_f64 * t650 * t1025 + 0.76905262301422242837e-2_f64 * t681 * t1025 - 0.76905262301422242837e-2_f64 * t1897 * t2909 + 0.76905262301422242837e-2_f64 * t2508 * t2912 + 0.76905262301422242837e-2_f64 * t270 * t2928 - 0.85450291446024714263e-3_f64 * t1841 * t2933 - 0.23071578690426672851e-1_f64 * t2508 * t2937 - 0.85450291446024714264e-3_f64 * t2545 + 0.64087718584518535698e-3_f64 * t2550 - 0.64087718584518535698e-3_f64 * t2556 + 0.64087718584518535698e-3_f64 * t2560 - 0.64087718584518535698e-3_f64 * t2565 - 0.10254034973522965712e-1_f64 * t650 * t1030 - 0.76905262301422242837e-2_f64 * t681 * t1030 + 0.76905262301422242837e-2_f64 * t1897 * t2951 + 0.85450291446024714263e-3_f64 * t1841 * t2955 + 0.15381052460284448567e-1_f64 * t2508 * t2960 - 0.76905262301422242837e-2_f64 * t270 * t2964;
    (t2964, t2967)
}
