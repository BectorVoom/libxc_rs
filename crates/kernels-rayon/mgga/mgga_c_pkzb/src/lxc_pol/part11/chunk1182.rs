//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1182/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1182(t12: f64, t10627: f64, t16425: f64, t600: f64, t10670: f64, t1769: f64, t1064: f64, t10760: f64, t10764: f64, t1430: f64, t207: f64, t2732: f64, t2735: f64, t28874: f64, t28877: f64, t28885: f64, t3510: f64, t439: f64, t8729: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t84 = t12 <= zeta_threshold;
    let t29024 = t10627 * t16425 * t600;
    let t29032 = t1769 * t10670;
    let t29049 = piecewise3(t84, 0.0_f64, -56.0_f64 / 81.0_f64 * t10760 * t439 + 16.0_f64 / 9.0_f64 * t3510 * t1430 + 8.0_f64 / 9.0_f64 * t2732 * t28874 - 4.0_f64 / 3.0_f64 * t2735 * t28877 - 2.0_f64 / 3.0_f64 * t1064 * t8729 - 2.0_f64 / 9.0_f64 * t10764 * t439 + 2.0_f64 / 3.0_f64 * t207 * t28885);
    (t29024, t29032, t29049)
}
