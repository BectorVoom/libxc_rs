//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 843/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk843(t2508: f64, t2580: f64, t44973: f64, t11595: f64, t2586: f64, t13486: f64, t7137: f64, t13507: f64, t7129: f64, t11603: f64, t2530: f64, t7226: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44976 = 0.15381052460284448567e-1_f64 * t2508 * t2580 * t44973;
    let t44990 = 0.23071578690426672851e-1_f64 * t2508 * t11595 * t2586;
    let t44992 = 0.30762104920568897135e-1_f64 * t7137 * t13486;
    let t44994 = 0.46143157380853345701e-1_f64 * t7129 * t13507;
    let t44995 = t11603 * t2530;
    let t44998 = 0.46143157380853345701e-1_f64 * t2508 * t7226 * t44995;
    (t44976, t44990, t44992, t44994, t44995, t44998)
}
