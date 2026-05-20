//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta204 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk836;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk837;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta204<F: Float>(t231: F, t2783: F, t4494: F, t2782: F, t1559: F, t72: F, t686: F, t2798: F, t225: F, t2718: F, t213: F, t1568: F, t233: F, t869: F, t689: F, t874: F, t822: F, t234: F, t2776: F, t2780: F, t2787: F, t2791: F, t2796: F, t2802: F, t2806: F, t2810: F, t2815: F, t4366: F, t4424: F, t4469: F, t820: F, t837: F, t879: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t4496, t4497, t4499, t4500, t4501, t4503, t4504, t4514) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk836::<F>(t231, t2783, t4494, t2782, t1559, t72, t686, t2798, t225, t2718, t213);
        let (t4518, t4519, t4522, t4526, t4533) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk837::<F>(t1568, t233, t869, t689, t72, t686, t874, t822, t1559, t213, t234, t2776, t2780, t2787, t2791, t2796, t2802, t2806, t2810, t2815, t4366, t4424, t4469, t4494, t4497, t4501, t4504, t4514, t820, t837, t879);
    (t4496, t4499, t4500, t4503, t4504, t4514, t4518, t4519, t4522, t4526, t4533)
}
