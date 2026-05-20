//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta419 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1586;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1587;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta419<F: Float>(t11710: F, t4787: F, t3091: F, t245: F, t4890: F, t3088: F, t3317: F, t1065: F, t1668: F, t372: F, t12131: F, t3095: F, t4823: F, t3096: F, t1087: F, t11773: F, t4801: F, t4181: F, t4786: F, t1062: F, t4857: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t15682, t15684, t15687, t15688, t15689, t15691, t15692) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1586::<F>(t11710, t4787, t3091, t245, t4890, t3088, t3317, t1065, t1668, t372, t12131, t3095);
        let (t15693, t15697, t15700, t15702, t15703, t15707) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1587::<F>(t15691, t15692, t372, t4823, t3096, t1087, t11773, t4801, t4181, t4786, t1062, t4857);
    (t15682, t15684, t15687, t15688, t15689, t15691, t15693, t15697, t15700, t15702, t15703, t15707)
}
