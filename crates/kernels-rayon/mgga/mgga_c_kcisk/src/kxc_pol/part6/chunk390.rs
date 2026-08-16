//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 390/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk390(t2541: f64, t752: f64, t196: f64, t2399: f64, t1919: f64, t1920: f64, t2063: f64, t2505: f64, t673: f64, t140: f64, t1470: f64, t1918: f64, t2517: f64, t2521: f64, t479: f64, t709: f64, t725: f64) -> (f64, f64, f64, f64, f64) {
    let t2542 = t2541 * t752;
    let t2543 = t2399 * t196;
    let t2551 = t1919 * t1920 * t2063;
    let t2554 = t673 * t2505;
    let t2558 = 0.619125e-2_f64 * t2543 * t709 + 0.9286875e-2_f64 * t725 * t2517 - 0.619125e-2_f64 * t725 * t2521 - t1918 - 0.26531111111111111111e-1_f64 * t1470 * t2551 - 0.39796666666666666666e-1_f64 * t140 * t479 * t2554;
    (t2542, t2543, t2551, t2554, t2558)
}
