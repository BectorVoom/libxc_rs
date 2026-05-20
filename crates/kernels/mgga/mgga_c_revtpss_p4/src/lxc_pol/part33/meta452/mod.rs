//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta452 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1643;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1644;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1645;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1646;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta452<F: Float>(t20823: F, t5268: F, t1042: F, t5265: F, t5274: F, t1774: F, t3362: F, t4181: F, t12787: F, t12916: F, t6689: F, t3718: F, t17661: F, t5401: F, t1214: F, t1715: F, t1250: F, t17353: F, t5052: F, t5406: F, t1794: F, t3617: F, t372: F, t5047: F, t3603: F, t5284: F, t5332: F, t3720: F, t12866: F, t17340: F, t17342: F, t17693: F, t17729: F, t3711: F, t5340: F, t11249: F, t6628: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t20914, t20917, t20923, t20926, t20927) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1643::<F>(t20823, t5268, t1042, t5265, t5274, t1774, t3362, t4181, t12787, t12916, t6689, t3718);
        let (t20929, t20934, t20938, t20941, t20945, t20946) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1644::<F>(t17661, t5401, t1214, t1715, t1250, t17353, t5052, t5406, t1794, t3617, t372, t5047);
        let (t20947, t20952, t20955) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1645::<F>(t20945, t20946, t3603, t5284, t5332, t3720, t12866, t17340, t17342, t17693, t17729, t20914, t20917, t20923, t20927, t20929, t20934, t20938, t20941, t3711, t5340);
        let t20956 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1646::<F>(t11249, t6628);
    (t20914, t20923, t20926, t20929, t20934, t20938, t20941, t20947, t20952, t20955, t20956)
}
