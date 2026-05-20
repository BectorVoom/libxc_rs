//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta405 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1776;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1777;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1778;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta405<F: Float>(t1269: F, t1770: F, t1811: F, t3555: F, t460: F, t5412: F, t17306: F, t487: F, t5219: F, t5216: F, t1204: F, t1209: F, t17288: F, t116: F, t4292: F, t5883: F, t648: F, t1501: F, t670: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t18005, t18037, t18054, t18059) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1776::<F>(t1269, t1770, t1811, t3555, t460, t5412, t17306, t487);
        let (t18062, t18065, t18087, t18097, t18114, t18207, t18220) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1777::<F>(t1269, t5219, t487, t5216, t1204, t1811, t1209, t5412, t17288, t116, t4292, t5883, t648);
        let t18227 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1778::<F>(t1501, t670);
    (t18005, t18037, t18054, t18059, t18062, t18065, t18087, t18097, t18114, t18207, t18220, t18227)
}
