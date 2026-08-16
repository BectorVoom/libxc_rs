//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1196/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1196(t42683: f64, t102: f64, t120: f64, t48562: f64, t10: f64, t48737: f64, t5825: f64, t48741: f64, t506: f64, t128: f64, t127: f64, t12929: f64, t25857: f64, t25866: f64, t3637: f64, t3665: f64, t42678: f64, t42681: f64, t42714: f64, t496: f64, t978: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t48777 = 0.116921e2_f64 * t42683;
    let t48780 = 0.2923025e1_f64 * t102 * t120 * t48562;
    let t48787 = t10 * t5825 * t48737;
    let t48791 = t10 * t506 * t48741;
    let t48795 = t10 * t128 * t48562;
    let t48807 = 0.587616e2_f64 * t42678 + 2.0_f64 / 3.0_f64 * t42681 - t48777 - t48780 - 0.146904e1_f64 * t127 * t506 * t48562 + 0.91406933333333333333e1_f64 * t25857 + 0.783488e1_f64 * t25866 + 30.0_f64 * t496 * t48787 + 9.0_f64 / 2.0_f64 * t496 * t48791 - t496 * t48795 / 2.0_f64 - 6.0_f64 * t42714 - 36.0_f64 * t496 * t10 * t3665 * t3637 + 6.0_f64 * t496 * t10 * t978 * t12929;
    (t48777, t48780, t48787, t48791, t48795, t48807)
}
