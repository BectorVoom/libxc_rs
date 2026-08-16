//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta180 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk922;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk923;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta180<F: Float>(t138: F, t9675: F, t9674: F, t4075: F, t556: F, t786: F, t4077: F, t676: F, t123: F, t1444: F, t2434: F, t3915: F, t1424: F, t4071: F, t4132: F, t9632: F, t9636: F, t9639: F, t9642: F, t9650: F, t9652: F, t9659: F, t9666: F, t9668: F, t9672: F) -> (F, F, F, F, F, F) {
        let (t9676, t9677, t9679, t9680, t9682, t9683, t9686, t9687) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk922::<F>(t138, t9675, t9674, t4075, t556, t786, t4077, t676, t123, t1444, t2434, t3915);
        let t9689 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk923::<F>(t1424, t4071, t4132, t9632, t9636, t9639, t9642, t9650, t9652, t9659, t9666, t9668, t9672, t9677, t9683, t9687);
    (t9676, t9679, t9680, t9682, t9686, t9689)
}
