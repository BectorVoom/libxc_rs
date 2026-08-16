//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta546 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1618;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1619;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta546<F: Float>(t157: F, t190: F, t87145: F, t49926: F, t49940: F, t76972: F, t61165: F, t39756: F, t39760: F, t39764: F, t39770: F, t39773: F, t39783: F, t39786: F, t6002: F, t61037: F, t61180: F, t76979: F, t39791: F, t39795: F, t39799: F, t39807: F, t39813: F, t39818: F, t39823: F, t40084: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t87640, t87641, t87642, t87643, t87644, t87645) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1618::<F>(t157, t190, t87145, t49926, t49940, t76972, t61165, t39756, t39760, t39764, t39770, t39773, t39783, t39786);
        let (t87649, t87650, t87651, t87652) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1619::<F>(t6002, t61037, t61180, t76979, t39791, t39795, t39799, t39807, t39813, t39818, t39823, t40084);
    (t87640, t87641, t87642, t87643, t87644, t87645, t87649, t87650, t87651, t87652)
}
