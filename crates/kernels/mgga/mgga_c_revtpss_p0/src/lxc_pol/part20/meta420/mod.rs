//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta420 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1565;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1566;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1567;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1568;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1569;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1570;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1571;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta420<F: Float>(t12291: F, t689: F, t12256: F, t2251: F, t2258: F, t12305: F, t128: F, t10326: F, t12281: F, t3360: F, t3363: F, t1120: F, t12286: F, t12268: F, t43808: F, t43810: F, t43814: F, t43817: F, t43823: F, t43826: F, t43828: F, t43830: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t43832 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1565::<F>(t12291, t689);
        let (t43835, t43837) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1566::<F>(t12256, t2251, t2258, t12305, t128);
        let (t43839, t43841) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1567::<F>(t10326, t12281, t128, t3360);
        let (t43843, t43845) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1568::<F>(t2258, t3363, t1120, t128);
        let (t43847, t43849) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1569::<F>(t10326, t12286, t1120, t128);
        let (t43852, t43854) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1570::<F>(t12268, t2251, t2258, t128, t3360);
        let t43856 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1571::<F>(t43808, t43810, t43814, t43817, t43823, t43826, t43828, t43830, t43832, t43837, t43841, t43845, t43849, t43854);
    (t43832, t43835, t43837, t43839, t43841, t43843, t43845, t43847, t43849, t43852, t43854, t43856)
}
