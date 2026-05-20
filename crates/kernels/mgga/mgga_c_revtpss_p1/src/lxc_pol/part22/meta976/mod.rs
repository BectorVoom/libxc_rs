//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta976 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3284;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3285;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta976<F: Float>(t50873: F, t40172: F, t14330: F, t18575: F, t2258: F, t14370: F, t18259: F, t18562: F, t2626: F, t18576: F, t50895: F, t5819: F, t606: F, t749: F, t1522: F, t49880: F, t50878: F, t40067: F, t40072: F, t40167: F, t40171: F, t40184: F, t61310: F, t61311: F, t61313: F, t61316: F, t61317: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t62269, t62270, t62273, t62275, t62277, t62279, t62282) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3284::<F>(t50873, t40172, t14330, t18575, t2258, t14370, t18259, t18562, t2626, t18576, t50895, t5819, t606, t749);
        let (t62283, t62285, t62286, t62287) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3285::<F>(t62282, t1522, t49880, t50878, t40067, t40072, t40167, t40171, t40184, t61310, t61311, t61313, t61316, t61317, t62269, t62270, t62273, t62275, t62277, t62279);
    (t62269, t62270, t62273, t62275, t62277, t62279, t62283, t62285, t62286, t62287)
}
