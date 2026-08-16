//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta419 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1562;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1563;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1564;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta419<F: Float>(t12331: F, t3391: F, t3399: F, t12322: F, t12346: F, t25273: F, t268: F, t404: F, t241: F, t281: F, t414: F, t39484: F, t403: F, t409: F, t3390: F, t12288: F, t698: F, t12316: F, t689: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t43808, t43810, t43813) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1562::<F>(t12331, t3391, t3399, t12322, t12346, t25273, t268, t404);
        let (t43814, t43816, t43817, t43822, t43823, t43825, t43826, t43828) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1563::<F>(t43813, t241, t281, t414, t39484, t403, t409, t3391, t3399, t3390, t12288, t698);
        let t43830 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1564::<F>(t12316, t689);
    (t43808, t43810, t43813, t43814, t43816, t43817, t43822, t43823, t43825, t43826, t43828, t43830)
}
