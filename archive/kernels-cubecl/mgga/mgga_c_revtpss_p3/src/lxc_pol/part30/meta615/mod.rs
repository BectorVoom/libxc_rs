//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta615 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2120;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2121;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta615<F: Float>(t1937: F, t98487: F, t27123: F, t6993: F, t25803: F, t7898: F, t2033: F, t47672: F, t1907: F, t4144: F, t28196: F, t27833: F, t7313: F, t3829: F, t28167: F, t8717: F, t25082: F, t28197: F, t73488: F, t13625: F, t33651: F, t25090: F, t28187: F, t7235: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t98489, t98491, t98494, t98499, t98501) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2120::<F>(t1937, t98487, t27123, t6993, t25803, t7898, t2033, t47672, t1907, t4144, t28196, t27833, t7313);
        let (t98522, t98525, t98528, t98530, t98532) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2121::<F>(t1907, t3829, t28167, t8717, t25082, t28197, t73488, t13625, t33651, t25090, t7898, t28187, t7235);
    (t98489, t98491, t98494, t98499, t98501, t98522, t98525, t98528, t98530, t98532)
}
