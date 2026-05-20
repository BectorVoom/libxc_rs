//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta818 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2929;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2930;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta818<F: Float>(t14085: F, t2435: F, t14104: F, t47520: F, t10069: F, t13731: F, t137: F, t14103: F, t47480: F, t9675: F, t14099: F, t2453: F, t9676: F, t14109: F, t9680: F, t9685: F, t5603: F, t9692: F, t1904: F, t689: F, t9634: F, t1364: F, t14067: F, t786: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t47834, t47837, t47839, t47844, t47856) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2929::<F>(t14085, t2435, t14104, t47520, t10069, t13731, t137, t14103, t47480, t9675, t14099, t2453);
        let (t47857, t47860, t47863, t47873, t47876) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2930::<F>(t47856, t9676, t14109, t9680, t9685, t5603, t9692, t1904, t689, t9634, t1364, t14067, t786);
    (t47834, t47837, t47839, t47844, t47856, t47857, t47860, t47863, t47873, t47876)
}
