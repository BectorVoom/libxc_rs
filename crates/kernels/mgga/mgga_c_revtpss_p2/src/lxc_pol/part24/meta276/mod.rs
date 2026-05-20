//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta276 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1050;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta276<F: Float>(t233: F, t6041: F, t869: F, t689: F, t251: F, t6016: F, t822: F, t6022: F, t72: F, t686: F, t10530: F, t6017: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t18688, t18689, t18690, t18699, t18714, t18718, t18719, t18720, t18725) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1050::<F>(t233, t6041, t869, t689, t251, t6016, t822, t6022, t72, t686, t10530, t6017);
    (t18688, t18689, t18690, t18699, t18714, t18718, t18719, t18720, t18725)
}
