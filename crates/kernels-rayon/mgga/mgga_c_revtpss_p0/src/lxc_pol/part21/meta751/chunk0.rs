//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2628/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2628(t47133: f64, t47135: f64, t13665: f64, t9572: f64, t1320: f64, t13680: f64, t47145: f64, t47147: f64, t47149: f64, t3863: f64, t5569: f64, t3860: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t48322 = 0.32530743900905219526e-1_f64 * t47133;
    let t48323 = 0.65061487801810439052e-1_f64 * t47135;
    let t48324 = t13665 * t9572;
    let t48325 = 0.32530743900905219526e-1_f64 * t48324;
    let t48326 = t1320 * t13680;
    let t48327 = 24.0_f64 * t48326;
    let t48328 = 0.51947577317044391277e2_f64 * t47145;
    let t48329 = 0.30762056574649219973e4_f64 * t47147;
    let t48330 = 12.0_f64 * t47149;
    let t48331 = t3863 * t5569;
    let t48332 = 96.0_f64 * t48331;
    let t48333 = t3860 * t5569;
    (t48322, t48323, t48325, t48327, t48328, t48329, t48330, t48332, t48333)
}
