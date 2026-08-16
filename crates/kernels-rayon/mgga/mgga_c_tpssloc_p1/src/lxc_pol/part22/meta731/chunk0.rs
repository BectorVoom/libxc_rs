//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2397/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2397(t13623: f64, t5705: f64, t17271: f64, t4378: f64, t21180: f64, t2798: f64, t896: f64, t2815: f64, t136: f64, t68569: f64, t908: f64, t41684: f64, t48946: f64, t48947: f64, t48956: f64, t59657: f64, t68442: f64, t68444: f64, t68446: f64, t68448: f64, t68479: f64, t68483: f64, t68486: f64, t68489: f64, t68492: f64, t68494: f64, t68498: f64, t68571: f64, t68577: f64, t68580: f64, t68583: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t68638 = t13623 * t5705;
    let t68640 = t4378 * t17271;
    let t68643 = t2798 * t21180 * t896;
    let t68646 = t2815 * t21180 * t896;
    let t68649 = t136 * t908 * t68569;
    let t68673 = 2.0_f64 / 3.0_f64 * t68442 + t68444 / 9.0_f64 + 10.0_f64 / 81.0_f64 * t68446 - 4.0_f64 / 9.0_f64 * t68448 + t48946 - t48947 - t48956 + 28.0_f64 / 81.0_f64 * t41684 - 80.0_f64 / 81.0_f64 * t68479 - 8.0_f64 * t68483 + 4.0_f64 * t68486 - 2.0_f64 / 3.0_f64 * t68489 - 2.0_f64 / 3.0_f64 * t68492 + 2.0_f64 / 9.0_f64 * t68494 - 2.0_f64 / 3.0_f64 * t68498 - 8.0_f64 / 27.0_f64 * t59657 - t68571 / 3.0_f64 + 8.0_f64 * t68577 - 6.0_f64 * t68580 + 2.0_f64 * t68583;
    (t68638, t68640, t68643, t68646, t68649, t68673)
}
