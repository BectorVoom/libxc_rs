//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta507 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2000;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta507<F: Float>(t17661: F, t5401: F, t1214: F, t1715: F, t1250: F, t17353: F, t5052: F, t5406: F, t1794: F, t3617: F, t372: F, t5047: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t20929, t20932, t20933, t20934, t20937, t20938, t20941, t20944, t20945, t20946) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2000::<F>(t17661, t5401, t1214, t1715, t1250, t17353, t5052, t5406, t1794, t3617, t372, t5047);
    (t20929, t20932, t20933, t20934, t20937, t20938, t20941, t20944, t20945, t20946)
}
