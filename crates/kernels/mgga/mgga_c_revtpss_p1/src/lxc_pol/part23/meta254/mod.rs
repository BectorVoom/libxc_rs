//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta254 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1440;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta254<F: Float>(t1444: F, t2438: F, t138: F, t9674: F, t4075: F, t556: F, t786: F) -> (F, F, F, F, F) {
        let (t9675, t9676, t9677, t9679, t9680) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1440::<F>(t1444, t2438, t138, t9674, t4075, t556, t786);
    (t9675, t9676, t9677, t9679, t9680)
}
