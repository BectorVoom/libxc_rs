//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1182/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1182(t36175: f64, t30689: f64, t5286: f64, t1165: f64, t2068: f64, t20972: f64, t7351: f64, t31759: f64, t31761: f64, t31763: f64, t31774: f64, t31782: f64, t31790: f64, t36147: f64, t36149: f64, t36152: f64, t36157: f64, t36160: f64, t36163: f64, t36165: f64, t36169: f64, t36173: f64) -> f64 {
    let t36176 = 0.94344276868812456204e-3_f64 * t36175;
    let t36177 = t30689 * t5286;
    let t36178 = 0.34299214494455789578e-2_f64 * t36177;
    let t36181 = t2068 * t1165 * t7351 * t20972;
    let t36183 = -0.3572834843172478081e-3_f64 * t31759 - 0.42874018118069736972e-3_f64 * t31761 - 0.21437009059034868486e-3_f64 * t31763 + t36147 / 16.0_f64 + t36149 / 48.0_f64 + t36152 + 0.16809375e0_f64 * t31774 + 0.84046875e-1_f64 * t31782 - 0.5603125e-1_f64 * t31790 - t36157 - 0.31448092289604152068e-3_f64 * t36160 + t36163 + 0.42874018118069736972e-3_f64 * t36165 + 0.42874018118069736972e-3_f64 * t36169 + 0.21437009059034868486e-3_f64 * t36173 - t36176 - t36178 - 0.94344276868812456204e-3_f64 * t36181;
    t36183
}
