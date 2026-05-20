//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta302 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1735;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1736;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1737;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta302<F: Float>(t10039: F, t869: F, t689: F, t2777: F, t4092: F, t2439: F, t1419: F, t3999: F, t3923: F, t555: F, t4003: F, t5744: F, t2782: F, t4086: F, t543: F, t123: F, t212: F, t2434: F, t4089: F, t138: F, t2438: F, t785: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t10040, t10041, t10043, t10044, t10049, t10059, t10061) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1735::<F>(t10039, t869, t689, t2777, t4092, t2439, t1419, t3999, t3923, t555, t4003, t5744);
        let (t10062, t10065, t10066, t10069) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1736::<F>(t10061, t2782, t10059, t4086, t543, t123, t212, t2434);
        let (t10070, t10073) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1737::<F>(t10069, t4089, t138, t2438, t785);
    (t10040, t10041, t10043, t10044, t10049, t10061, t10062, t10065, t10066, t10069, t10070, t10073)
}
