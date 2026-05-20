//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta584 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2450;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta584<F: Float>(t6041: F, t822: F, t6022: F, t72: F, t686: F, t10530: F, t10645: F, t10647: F, t10651: F, t14558: F, t14564: F, t14570: F, t18616: F, t18632: F, t18657: F, t18690: F, t18699: F, t213: F, t234: F, t2815: F, t4424: F, t4494: F, t4504: F, t4514: F, t4526: F, t6017: F, t820: F, t837: F, t879: F) -> (F, F, F, F) {
        let (t18714, t18718, t18719, t18722) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2450::<F>(t6041, t822, t6022, t72, t686, t10530, t10645, t10647, t10651, t14558, t14564, t14570, t18616, t18632, t18657, t18690, t18699, t213, t234, t2815, t4424, t4494, t4504, t4514, t4526, t6017, t820, t837, t879);
    (t18714, t18718, t18719, t18722)
}
