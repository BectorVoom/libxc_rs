//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta312 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1099;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta312<F: Float>(t6800: F, t749: F, t512: F, t177: F, t762: F, t1877: F, t73: F, t4010: F, t6836: F, t1412: F, t6816: F, t221: F, t4019: F, t6844: F) -> (F, F, F, F, F, F, F, F) {
        let (t22195, t22196, t22212, t22213, t22229, t22236, t22245, t22259) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1099::<F>(t6800, t749, t512, t177, t762, t1877, t73, t4010, t6836, t1412, t6816, t221, t4019, t6844);
    (t22195, t22196, t22212, t22213, t22229, t22236, t22245, t22259)
}
