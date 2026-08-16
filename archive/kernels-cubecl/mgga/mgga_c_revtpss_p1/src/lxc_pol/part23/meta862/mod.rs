//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta862 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2752;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2753;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta862<F: Float>(t2608: F, t512: F, t6800: F, t177: F, t21931: F, t762: F, t1320: F, t22193: F, t22461: F, t4147: F, t749: F, t22212: F, t2516: F, t72: F, t757: F, t6922: F, t9593: F, t22185: F, t2619: F, t22404: F, t3920: F, t1445: F, t22445: F, t689: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t73350, t73352, t73374, t73407, t73476, t73481) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2752::<F>(t2608, t512, t6800, t177, t21931, t762, t1320, t22193, t22461, t4147, t749, t22212, t2516);
        let (t73493, t73499, t73515, t73587, t73590) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2753::<F>(t21931, t72, t757, t6922, t9593, t22185, t2619, t22404, t3920, t1445, t22445, t689);
    (t73350, t73352, t73374, t73407, t73476, t73481, t73493, t73499, t73515, t73587, t73590)
}
