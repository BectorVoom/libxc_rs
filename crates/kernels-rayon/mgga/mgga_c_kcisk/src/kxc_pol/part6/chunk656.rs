//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 656/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk656(t9046: f64, t9093: f64, t1908: f64, t5360: f64, t6756: f64, t8512: f64, t8516: f64, t8520: f64, t2604: f64, t1974: f64, t5380: f64, t5387: f64, t6823: f64, t8525: f64, t8527: f64, t8559: f64, t8561: f64, t8565: f64, t8568: f64, t8571: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9094 = t9046 + t9093;
    let t9095 = t1908 * t9094;
    let t9103 = t5360 + 0.11415555555555555555e-1_f64 * t6756 - 0.11415555555555555555e-1_f64 * t8512 + 0.34246666666666666666e-1_f64 * t8516 - 0.17123333333333333333e-1_f64 * t8520;
    let t9108 = t2604 * t2604;
    let t9109 = t9108 * t1974;
    let t9124 = -0.17648625e1_f64 * t8525 + 0.3529725e1_f64 * t8527 + t5380 + 0.34431666666666666666e0_f64 * t6756 - 0.34431666666666666667e0_f64 * t8512 + 0.103295e1_f64 * t8516 - 0.516475e0_f64 * t8520 + 0.31558125e0_f64 * t8559 + 0.6311625e0_f64 * t8561 + t5387 + 0.13892666666666666667e0_f64 * t6823 - 0.34731666666666666667e-1_f64 * t8565 + 0.20839e0_f64 * t8568 - 0.104195e0_f64 * t8571;
    (t9094, t9095, t9103, t9108, t9109, t9124)
}
