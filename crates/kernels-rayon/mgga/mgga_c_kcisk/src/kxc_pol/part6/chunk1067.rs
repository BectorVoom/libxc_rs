//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1067/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1067(t2105: f64, t27613: f64, t2293: f64, t8365: f64, t2292: f64, t27584: f64, t2297: f64, t7819: f64, t8349: f64, t4463: f64, t19100: f64, t25590: f64, t25601: f64, t25609: f64, t25696: f64, t25699: f64, t25701: f64, t30569: f64, t30572: f64, t30582: f64, t30585: f64, t30606: f64, t30608: f64, t30610: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31509 = t27613 * t2105;
    let t31512 = t2293 * t8365;
    let t31515 = t27584 * t2292;
    let t31518 = t2297 * t7819;
    let t31525 = t8349 * t2292;
    let t31526 = t31525 * t4463;
    let t31543 = -0.103295e1_f64 * t30569 + 0.309885e1_f64 * t30572 - 0.68863333333333333332e0_f64 * t19100 + 0.34431666666666666666e0_f64 * t25590 - 0.103295e1_f64 * t25601 + 0.51647499999999999999e0_f64 * t25609 - 0.41678000000000000001e0_f64 * t25696 + 0.20839e0_f64 * t25699 + 0.69463333333333333335e-1_f64 * t25701 - 0.104195e0_f64 * t30582 + 0.62517e0_f64 * t30585 + 0.6311625e0_f64 * t30606 + 0.3529725e1_f64 * t30608 - 0.52945875e1_f64 * t30610;
    (t31509, t31512, t31515, t31518, t31525, t31526, t31543)
}
