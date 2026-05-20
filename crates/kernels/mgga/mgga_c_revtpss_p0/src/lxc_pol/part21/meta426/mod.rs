//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta426 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1920;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta426<F: Float>(t1892: F, t785: F, t1358: F, t2439: F, t1903: F, t4075: F, t1444: F, t556: F, t2782: F, t212: F, t5710: F, t689: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t13725, t13726, t13727, t13729, t13730, t13731, t13733, t13734, t13735, t13737) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1920::<F>(t1892, t785, t1358, t2439, t1903, t4075, t1444, t556, t2782, t212, t5710, t689);
    (t13725, t13726, t13727, t13729, t13730, t13731, t13733, t13734, t13735, t13737)
}
