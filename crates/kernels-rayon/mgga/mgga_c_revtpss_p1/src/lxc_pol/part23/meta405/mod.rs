//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta405 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1776;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1777;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1778;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta405(t1269: f64, t1770: f64, t1811: f64, t3555: f64, t460: f64, t5412: f64, t17306: f64, t487: f64, t5219: f64, t5216: f64, t1204: f64, t1209: f64, t17288: f64, t116: f64, t4292: f64, t5883: f64, t648: f64, t1501: f64, t670: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18005, t18037, t18054, t18059) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1776(t1269, t1770, t1811, t3555, t460, t5412, t17306, t487);
        let (t18062, t18065, t18087, t18097, t18114, t18207, t18220) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1777(t1269, t5219, t487, t5216, t1204, t1811, t1209, t5412, t17288, t116, t4292, t5883, t648);
        let t18227 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1778(t1501, t670);
    (t18005, t18037, t18054, t18059, t18062, t18065, t18087, t18097, t18114, t18207, t18220, t18227)
}
