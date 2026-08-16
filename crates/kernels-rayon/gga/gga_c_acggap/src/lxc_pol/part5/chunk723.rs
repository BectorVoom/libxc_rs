//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 723/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk723(t1165: f64, t1532: f64, t5284: f64, t3194: f64, t129: f64, t145: f64, t4875: f64, t5: f64, t1173: f64, t127: f64, t3816: f64, t418: f64, t5253: f64, t5255: f64, t5260: f64, t5263: f64, t5267: f64, t5272: f64, t5277: f64, t5281: f64) -> (f64, f64, f64, f64) {
    let t5286 = t1165 * t1532 * t5284;
    let t5288 = 0.17149607247227894789e-2_f64 * t3194 * t5286;
    let t5291 = t129 * t5 * t4875 * t145;
    let t5294 = 35.0_f64 / 216.0_f64 * t3816 + t5253 + 0.85748036236139473944e-2_f64 * t418 * t5255 - 0.34299214494455789578e-2_f64 * t418 * t5260 - 0.80031500487063509014e-2_f64 * t5263 - 0.85748036236139473945e-2_f64 * t418 * t5267 + 0.34299214494455789578e-2_f64 * t1173 * t5272 - 0.34299214494455789578e-2_f64 * t1173 * t5277 + 0.17149607247227894789e-2_f64 * t1173 * t5281 - t5288 + t127 * t5291 / 96.0_f64;
    (t5286, t5288, t5291, t5294)
}
