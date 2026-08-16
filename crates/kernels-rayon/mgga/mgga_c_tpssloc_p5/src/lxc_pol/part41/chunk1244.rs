//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1244/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1244(t19743: f64, t19744: f64, t5250: f64, t5287: f64, t5348: f64, t1336: f64, t16047: f64, t19654: f64, t19658: f64, t19661: f64, t19668: f64, t19674: f64, t19733: f64, t19736: f64, t19740: f64, t3777: f64, t5234: f64, t5334: f64, t5336: f64, t5349: f64, t6448: f64, t6451: f64, t6454: f64, t6456: f64) -> f64 {
    let t19745 = t19743 * t19744;
    let t19748 = t19743 * t5250;
    let t19752 = t5348 * t5287;
    let t19755 = -t1336 * t19658 + 2.0_f64 * t1336 * t19668 - t1336 * t19674 - t1336 * t19733 - 2.0_f64 * t1336 * t19752 - 6.0_f64 * t16047 * t19745 + 4.0_f64 * t19654 * t5336 + 2.0_f64 * t19661 * t5334 + 4.0_f64 * t19736 * t5334 + 4.0_f64 * t19740 * t5334 + 6.0_f64 * t19748 * t5334 + 2.0_f64 * t3777 * t6448 - 2.0_f64 * t3777 * t6451 - t3777 * t6454 - t3777 * t6456 - 2.0_f64 * t5234 * t5349;
    t19755
}
