//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta348 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1211;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta348(t1045: f64, t23820: f64, t373: f64, t1042: f64, t11632: f64, t23641: f64, t11250: f64, t1668: f64, t6244: f64, t3117: f64, t1469: f64, t5825: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23822, t23823, t23829, t23830, t23833, t23834, t23837, t23838, t23839, t23842) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1211(t1045, t23820, t373, t1042, t11632, t23641, t11250, t1668, t6244, t3117, t1469, t5825);
    (t23822, t23823, t23829, t23830, t23833, t23834, t23837, t23838, t23839, t23842)
}
