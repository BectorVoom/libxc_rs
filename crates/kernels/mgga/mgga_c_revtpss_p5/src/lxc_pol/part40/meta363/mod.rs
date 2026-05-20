//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta363 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1271;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1272;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1273;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta363<F: Float>(t15154: F, t2908: F, t141: F, t15158: F, t930: F, t4625: F, t698: F, t4622: F, t15130: F, t15137: F, t15142: F, t15147: F, t15151: F, t15156: F, t15160: F, t15135: F, t11341: F, t15140: F, t15145: F, t15149: F, t1593: F, t2435: F, t4584: F, t689: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t15163, t15166, t15168, t15170, t15173, t15175) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1271::<F>(t15154, t2908, t141, t15158, t930, t4625, t698, t4622, t15130, t15137, t15142, t15147, t15151, t15156, t15160);
        let (t15178, t15181, t15184, t15187, t15189) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1272::<F>(t15135, t2908, t141, t11341, t15140, t15145, t930, t15149, t1593, t2435);
        let t15191 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1273::<F>(t4584, t689);
    (t15163, t15166, t15168, t15170, t15173, t15175, t15178, t15181, t15184, t15187, t15189, t15191)
}
