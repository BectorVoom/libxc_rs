//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta503 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1509;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1510;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta503(t23253: f64, t40348: f64, t10777: f64, t10779: f64, t1559: f64, t5984: f64, t10905: f64, t23275: f64, t6035: f64, t61956: f64, t40725: f64, t5988: f64, t14923: f64, t23301: f64, t125: f64, t23114: f64, t61715: f64, t14931: f64, t23334: f64, t10811: f64, t23331: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t76647, t76672, t76677, t76689, t76701) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1509(t23253, t40348, t10777, t10779, t1559, t5984, t10905, t23275, t6035, t61956, t40725, t5988);
        let (t76703, t76705, t76720, t76738, t76740) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1510(t14923, t23301, t125, t23114, t10777, t10779, t6035, t61715, t14931, t23334, t61956, t10811, t23331);
    (t76647, t76672, t76677, t76689, t76701, t76703, t76705, t76720, t76738, t76740)
}
