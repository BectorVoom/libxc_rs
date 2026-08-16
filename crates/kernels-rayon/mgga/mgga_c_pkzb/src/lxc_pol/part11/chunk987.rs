//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 987/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk987(t10800: f64, t5511: f64, t2754: f64, t3532: f64, t10769: f64, t5520: f64, t7357: f64, t9148: f64, t665: f64, t5547: f64, t2765: f64, t672: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10801 = t5511 * t10800;
    let t10803 = t2754 * t3532;
    let t10806 = -t5520 + 4.0_f64 / 3.0_f64 * t7357 - t9148 + t10769;
    let t10807 = t665 * t10806;
    let t10812 = t5547 * t10800;
    let t10814 = t2765 * t3532;
    let t10816 = t672 * t10806;
    (t10801, t10803, t10806, t10807, t10812, t10814, t10816)
}
