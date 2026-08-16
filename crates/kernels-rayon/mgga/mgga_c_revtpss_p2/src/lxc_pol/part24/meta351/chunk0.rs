//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1215/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1215(t19501: f64, t23898: f64, t3092: f64, t6266: f64, t19611: f64, t357: f64, t4781: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t23899 = t19501 * t23898;
    let t23900 = t3092 * t23899;
    let t23903 = t19501 * t6266;
    let t23904 = t3092 * t23903;
    let t23907 = t19611 * t6266;
    let t23908 = t3092 * t23907;
    let t23911 = t4781 * t357;
    (t23899, t23900, t23903, t23904, t23907, t23908, t23911)
}
