//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta341 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1643;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta341<F: Float>(t1532: F, t2609: F, t2398: F, t4305: F, t177: F, t4392: F, t762: F, t10605: F, t162: F, t2626: F, t4398: F, t10439: F) -> (F, F, F, F, F, F, F) {
        let (t14312, t14317, t14322, t14324, t14325, t14328, t14330) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1643::<F>(t1532, t2609, t2398, t4305, t177, t4392, t762, t10605, t162, t2626, t4398, t10439);
    (t14312, t14317, t14322, t14324, t14325, t14328, t14330)
}
