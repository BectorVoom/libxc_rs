//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 879/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk879<F: Float>(t13489: F, t2549: F, t11608: F, t1897: F, t2580: F, t7068: F, t1035: F, t10673: F, t13527: F, t13531: F, t13539: F, t2508: F, t3049: F, t3433: F, t44967: F, t44972: F, t44976: F, t44990: F, t44992: F, t44994: F, t44998: F, t45000: F, t45001: F, t45009: F, t7129: F, t7137: F, t740: F, t779: F) -> F {
    let t45010 = t2549 * t13489;
    let t45015 = F::new(0.15381052460284448567e-1) * t1897 * t2580 * t11608 * t7068;
    let t45016 = F::new(0.30762104920568897134e-1) * t7129 * t13539 + F::new(0.30762104920568897134e-1) * t2508 * t2580 * t44967 + t44972 + t44976 + F::new(0.76905262301422242837e-2) * t2508 * t779 * t13527 + F::new(0.15381052460284448567e-1) * t7129 * t13531 + F::new(0.15381052460284448567e-1) * t2508 * t3049 * t3433 + F::new(0.15381052460284448567e-1) * t2508 * t1035 * t10673 - t44990 - t44992 - t44994 - t44998 + t45000 - F::new(0.23071578690426672851e-1) * t2508 * t45001 * t740 + F::new(0.20508069947045931423e-1) * t7137 * t13531 + t45009 - F::new(0.96131577876777803546e-3) * t45010 - t45015;
    t45016
}
