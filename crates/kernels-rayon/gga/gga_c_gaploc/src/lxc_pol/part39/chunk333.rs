//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 333/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk333(t2657: f64, t959: f64, t1457: f64, t2572: f64, t1966: f64, t1991: f64, t1998: f64, t2004: f64, t2009: f64, t2103: f64, t2598: f64, t2601: f64, t2605: f64, t2608: f64, t2613: f64, t2619: f64, t2621: f64, t2625: f64, t2629: f64, t2631: f64, t2635: f64, t2638: f64, t2639: f64, t2642: f64, t2646: f64, t2649: f64, t2654: f64, t780: f64, t807: f64, t813: f64, t833: f64) -> (f64, f64, f64) {
    let t2658 = t2657 * t959;
    let t2660 = t1457 * t2572;
    let t2663 = 0.30674340763136599741e1_f64 * t833 * t2598 + 0.14896037479937677779e-1_f64 * t2601 - 0.14896037479937677779e-1_f64 * t2605 + 0.12780975317973583226e0_f64 * t2608 - 0.14896037479937677779e-1_f64 * t2613 + 0.95857314884801874192e-1_f64 * t2619 + 0.51123901271894332902e0_f64 * t1991 * t2621 - 0.51123901271894332902e0_f64 * t1966 * t2625 - 0.29792074959875355558e-1_f64 * t2629 + 0.71500979903700853338e0_f64 * t2103 * t2631 - 0.46011511144704899612e1_f64 * t813 * t2635 - 0.10725146985555128001e1_f64 * t2638 * t2639 + 0.23005755572352449806e1_f64 * t807 * t2642 - 0.23005755572352449806e1_f64 * t1998 * t2646 - 0.35750489951850426669e0_f64 * t2649 * t2009 + 0.35750489951850426669e0_f64 * t780 * t2654 + 0.14896037479937677779e-1_f64 * t2658 + 0.35750489951850426669e0_f64 * t2004 * t2660;
    (t2658, t2660, t2663)
}
