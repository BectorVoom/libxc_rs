//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 837/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk837(t3406: f64, t568: f64, t581: f64, t1706: f64, t2592: f64, t5225: f64, t5265: f64, t6873: f64, t6885: f64, t6894: f64, t6914: f64, t6928: f64, t6933: f64, t8921: f64, t8924: f64, t8926: f64, t8931: f64, t8935: f64) -> (f64, f64) {
    let t8939 = t581 * t3406 * t568;
    let t8944 = 0.85748036236139473944e-3_f64 * t2592 * t8921 - 7.0_f64 / 48.0_f64 * t8924 + 7.0_f64 / 144.0_f64 * t8926 - 0.80031500487063509016e-2_f64 * t6873 - 0.80031500487063509015e-2_f64 * t6885 - t6894 - t5225 * t8931 / 4.0_f64 + t1706 * t8935 / 8.0_f64 + t1706 * t8939 / 16.0_f64 - 35.0_f64 / 216.0_f64 * t5265 - t6914 + t6928 - 35.0_f64 / 108.0_f64 * t6933;
    (t8939, t8944)
}
