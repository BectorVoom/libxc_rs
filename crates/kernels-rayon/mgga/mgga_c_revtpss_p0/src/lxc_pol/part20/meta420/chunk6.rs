//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1571/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1571(t43808: f64, t43810: f64, t43814: f64, t43817: f64, t43823: f64, t43826: f64, t43828: f64, t43830: f64, t43832: f64, t43837: f64, t43841: f64, t43845: f64, t43849: f64, t43854: f64) -> f64 {
    let t43856 = -0.247573125e0_f64 * t43808 + 0.3300975e0_f64 * t43810 + t43814 + t43817 - 0.485484375e1_f64 * t43823 - 0.3883875e1_f64 * t43826 - 0.132456e1_f64 * t43828 - 0.24154e1_f64 * t43830 + 0.80513333333333333333e0_f64 * t43832 + 0.20128333333333333334e1_f64 * t43837 - 0.80513333333333333332e0_f64 * t43841 + 0.108693e2_f64 * t43845 + 0.24154e1_f64 * t43849 - 0.72462e1_f64 * t43854;
    t43856
}
