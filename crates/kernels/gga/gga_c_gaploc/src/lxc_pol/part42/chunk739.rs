//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 739/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk739<F: Float>(t2508: F, t2580: F, t44973: F, t11595: F, t2586: F, t13486: F, t7137: F, t13507: F, t7129: F, t11603: F, t2530: F, t7226: F, t13556: F, t1897: F, t7671: F, t13489: F, t2549: F) -> (F, F, F, F, F, F, F, F, F) {
    let t44976 = 0.15381052460284448567e-1 * t2508 * t2580 * t44973;
    let t44990 = 0.23071578690426672851e-1 * t2508 * t11595 * t2586;
    let t44992 = 0.30762104920568897135e-1 * t7137 * t13486;
    let t44994 = 0.46143157380853345701e-1 * t7129 * t13507;
    let t44995 = t11603 * t2530;
    let t44998 = 0.46143157380853345701e-1 * t2508 * t7226 * t44995;
    let t45000 = 0.20508069947045931423e-1 * t7137 * t13556;
    let t45009 = 0.23071578690426672851e-1 * t1897 * t11595 * t7671;
    let t45010 = t2549 * t13489;
    (t44976, t44990, t44992, t44994, t44995, t44998, t45000, t45009, t45010)
}
