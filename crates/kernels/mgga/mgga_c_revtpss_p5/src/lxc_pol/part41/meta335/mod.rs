//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta335 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1134;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1135;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1136;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta335<F: Float>(t15045: F, t689: F, t4470: F, t786: F, t789: F, t4534: F, t779: F, t2435: F, t4322: F, t1596: F, t2873: F, t1614: F, t2942: F, t1606: F, t2439: F, t4580: F, t4575: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t15047, t15050, t15062, t15063, t15101, t15104) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1134::<F>(t15045, t689, t4470, t786, t789, t4534, t779, t2435, t4322, t1596, t2873, t1614, t2942);
        let (t15123, t15125) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1135::<F>(t1606, t2439, t4580, t689);
        let t15127 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1136::<F>(t4575, t689);
    (t15047, t15050, t15062, t15063, t15101, t15104, t15123, t15125, t15127)
}
