//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta536 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1961;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1962;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta536<F: Float>(t29135: F, t7642: F, t2148: F, t1294: F, t8208: F, t26969: F, t1775: F, t1829: F, t2149: F, t2152: F, t27008: F, t27011: F, t27025: F, t29111: F, t29119: F, t29124: F, t29129: F, t29132: F, t5246: F, t7602: F, t7643: F, t7645: F, t7648: F, t7651: F, t7654: F, t7659: F, t7662: F, t7666: F, t8198: F, t8205: F, t8217: F, t1794: F, t2142: F, t73: F) -> (F, F, F, F, F, F, F) {
        let (t29136, t29141, t29148, t29149, t29154) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1961::<F>(t29135, t7642, t2148, t1294, t8208, t26969, t1775, t1829, t2149, t2152, t27008, t27011, t27025, t29111, t29119, t29124, t29129, t29132, t5246, t7602, t7643, t7645, t7648, t7651, t7654, t7659, t7662, t7666, t8198, t8205, t8217);
        let (t29157, t29158) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1962::<F>(t1794, t2142, t73);
    (t29136, t29141, t29148, t29149, t29154, t29157, t29158)
}
