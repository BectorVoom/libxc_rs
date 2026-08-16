//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta90 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk646;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk647;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta90<F: Float>(t159: F, t215: F, t10: F, t17: F, t576: F, t580: F, t15: F, t22: F, t11: F, t14: F, t584: F, t588: F, t20: F, t27: F, t12: F, t19: F, t592: F, t596: F, t21: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1941, t2219, t2221, t2223, t2224, t2226, t2228) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk646::<F>(t159, t215, t10, t17, t576, t580, t15, t22, t11, t14, t584, t588);
        let (t2230, t2231, t2233, t2235, t2236) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk647::<F>(t20, t27, t12, t19, t592, t596, t21);
    (t1941, t2219, t2221, t2223, t2224, t2226, t2228, t2230, t2231, t2233, t2235, t2236)
}
