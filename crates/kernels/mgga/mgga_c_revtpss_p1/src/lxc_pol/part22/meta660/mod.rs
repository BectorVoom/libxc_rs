//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta660 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2615;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2616;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2617;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta660<F: Float>(t1250: F, t5052: F, t17353: F, t17661: F, t5406: F, t1794: F, t3617: F, t372: F, t5047: F, t3603: F, t5284: F, t5332: F, t3720: F, t12866: F, t17340: F, t17342: F, t17693: F, t17729: F, t20914: F, t20917: F, t20923: F, t20927: F, t20929: F, t20934: F, t3711: F, t5340: F, t11249: F, t6628: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t20937, t20938, t20941, t20944, t20945, t20946, t20947, t20950, t20951) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2615::<F>(t1250, t5052, t17353, t17661, t5406, t1794, t3617, t372, t5047, t3603, t5284, t5332);
        let (t20952, t20955) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2616::<F>(t20951, t3720, t12866, t17340, t17342, t17693, t17729, t20914, t20917, t20923, t20927, t20929, t20934, t20938, t20941, t20947, t3711, t5340);
        let t20956 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2617::<F>(t11249, t6628);
    (t20937, t20938, t20941, t20944, t20945, t20946, t20947, t20950, t20951, t20952, t20955, t20956)
}
