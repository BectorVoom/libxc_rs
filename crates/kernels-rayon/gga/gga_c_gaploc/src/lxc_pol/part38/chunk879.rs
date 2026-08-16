//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 879/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk879(t13489: f64, t2549: f64, t11608: f64, t1897: f64, t2580: f64, t7068: f64, t1035: f64, t10673: f64, t13527: f64, t13531: f64, t13539: f64, t2508: f64, t3049: f64, t3433: f64, t44967: f64, t44972: f64, t44976: f64, t44990: f64, t44992: f64, t44994: f64, t44998: f64, t45000: f64, t45001: f64, t45009: f64, t7129: f64, t7137: f64, t740: f64, t779: f64) -> f64 {
    let t45010 = t2549 * t13489;
    let t45015 = 0.15381052460284448567e-1_f64 * t1897 * t2580 * t11608 * t7068;
    let t45016 = 0.30762104920568897134e-1_f64 * t7129 * t13539 + 0.30762104920568897134e-1_f64 * t2508 * t2580 * t44967 + t44972 + t44976 + 0.76905262301422242837e-2_f64 * t2508 * t779 * t13527 + 0.15381052460284448567e-1_f64 * t7129 * t13531 + 0.15381052460284448567e-1_f64 * t2508 * t3049 * t3433 + 0.15381052460284448567e-1_f64 * t2508 * t1035 * t10673 - t44990 - t44992 - t44994 - t44998 + t45000 - 0.23071578690426672851e-1_f64 * t2508 * t45001 * t740 + 0.20508069947045931423e-1_f64 * t7137 * t13531 + t45009 - 0.96131577876777803546e-3_f64 * t45010 - t45015;
    t45016
}
