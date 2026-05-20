//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta764 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2712;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2713;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta764<F: Float>(t4186: F, t4401: F, t606: F, t749: F, t39737: F, t39766: F, t10433: F, t4311: F, t10489: F, t2403: F, t39741: F, t39744: F, t39747: F, t39750: F, t39756: F, t39760: F, t39764: F, t39770: F, t4546: F, t1522: F, t40158: F, t14362: F, t9575: F, t123: F, t2630: F, t4392: F, t4398: F, t9318: F, t11231: F, t14330: F, t4402: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t49912, t49913, t49918, t49920, t49921) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2712::<F>(t4186, t4401, t606, t749, t39737, t39766, t10433, t4311, t10489, t2403, t39741, t39744, t39747, t39750, t39756, t39760, t39764, t39770, t4546);
        let (t49925, t49927, t49930, t49941, t49944) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2713::<F>(t1522, t40158, t14362, t9575, t123, t2630, t4392, t4398, t9318, t11231, t14330, t4402);
    (t49912, t49913, t49918, t49920, t49921, t49925, t49927, t49930, t49941, t49944)
}
