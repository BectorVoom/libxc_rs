//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1003/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1003<F: Float>(t1847: F, t22075: F, t587: F, t601: F, t1867: F, t6419: F, t6820: F, t1859: F, t6427: F, t1849: F, t6424: F, t588: F) -> (F, F, F, F, F, F) {
    let t22079 = F::new(0.35089340384731224426e1) * t601 * t1847 * t22075 * t587;
    let t22095 = t1867 * t6419;
    let t22098 = F::new(0.69263023597503453196e2) * t601 * t6820 * t22095;
    let t22100 = t6427 * t1859;
    let t22103 = F::new(0.61523382126046769581e4) * t601 * t6424 * t1849 * t22100;
    let t22107 = F::new(0.46785787179641632568e1) * t601 * t1847 * t6419 * t588;
    (t22079, t22095, t22098, t22100, t22103, t22107)
}
