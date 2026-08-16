//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta652 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2439;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta652<F: Float>(t225: F, t42277: F, t366: F, t11792: F, t3215: F, t11951: F, t3224: F, t1025: F, t11809: F, t127: F, t371: F, t1053: F, t11782: F) -> (F, F, F, F, F, F) {
        let (t42278, t42279, t42282, t42284, t42288, t42290) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2439::<F>(t225, t42277, t366, t11792, t3215, t11951, t3224, t1025, t11809, t127, t371, t1053, t11782);
    (t42278, t42279, t42282, t42284, t42288, t42290)
}
