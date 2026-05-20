//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta750 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2539;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta750<F: Float>(t52037: F, t52126: F, t3011: F, t4682: F, t11506: F, t1626: F, t1609: F, t2924: F, t51973: F, t52035: F, t2942: F, t4644: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t52598, t52623, t52637, t52642, t52645, t52701, t52751, t52774, t52783, t52784, t52809) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2539::<F>(t52037, t52126, t3011, t4682, t11506, t1626, t1609, t2924, t51973, t52035, t2942, t4644);
    (t52598, t52623, t52637, t52642, t52645, t52701, t52751, t52774, t52783, t52784, t52809)
}
