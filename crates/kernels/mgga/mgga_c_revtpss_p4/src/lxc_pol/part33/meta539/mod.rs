//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta539 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1907;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1908;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1909;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1910;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta539<F: Float>(t2148: F, t5412: F, t1032: F, t1811: F, t7642: F, t1294: F, t8208: F, t26969: F, t1775: F, t1829: F, t2149: F, t2152: F, t27008: F, t27011: F, t27025: F, t29111: F, t29119: F, t29124: F, t29129: F, t5246: F, t7602: F, t7643: F, t7645: F, t7648: F, t7651: F, t7654: F, t7659: F, t7662: F, t7666: F, t8198: F, t8205: F, t8217: F) -> (F, F, F, F, F, F) {
        let (t29132, t29135) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1907::<F>(t2148, t5412, t1032, t1811);
        let t29136 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1908::<F>(t29135, t7642);
        let t29141 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1909::<F>(t2148, t29135);
        let (t29149, t29154) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1910::<F>(t1294, t8208, t26969, t1775, t1829, t2149, t2152, t27008, t27011, t27025, t29111, t29119, t29124, t29129, t29132, t29136, t29141, t5246, t7602, t7643, t7645, t7648, t7651, t7654, t7659, t7662, t7666, t8198, t8205, t8217);
    (t29132, t29135, t29136, t29141, t29149, t29154)
}
