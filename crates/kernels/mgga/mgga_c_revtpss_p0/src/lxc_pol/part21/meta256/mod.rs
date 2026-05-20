//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta256 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1447;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1448;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta256<F: Float>(t3906: F, t9664: F, t1357: F, t4132: F, t689: F, t4131: F, t676: F, t123: F, t3915: F, t2453: F, t3914: F, t1444: F, t2438: F, t138: F, t4075: F, t556: F, t786: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t9666, t9667, t9668, t9671, t9672, t9674) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1447::<F>(t3906, t9664, t1357, t4132, t689, t4131, t676, t123, t3915, t2453, t3914);
        let (t9675, t9676, t9677, t9679, t9680) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1448::<F>(t1444, t2438, t138, t9674, t4075, t556, t786);
    (t9666, t9667, t9668, t9671, t9672, t9674, t9675, t9676, t9677, t9679, t9680)
}
