//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 621/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk621(t1685: f64, t4761: f64, t8590: f64, t4769: f64, t4776: f64, t6756: f64, t6823: f64, t8512: f64, t8516: f64, t8520: f64, t8525: f64, t8527: f64, t8559: f64, t8561: f64, t8565: f64, t8568: f64, t8571: f64) -> (f64, f64) {
    let t8592 = t4761 * t8590 * t1685;
    let t8607 = -0.1294625e1_f64 * t8525 + 0.258925e1_f64 * t8527 + t4769 + 0.20128333333333333334e0_f64 * t6756 - 0.20128333333333333333e0_f64 * t8512 + 0.60385e0_f64 * t8516 - 0.301925e0_f64 * t8520 + 0.82524375e-1_f64 * t8559 + 0.16504875e0_f64 * t8561 + t4776 + 0.11038e0_f64 * t6823 - 0.27595e-1_f64 * t8565 + 0.16557e0_f64 * t8568 - 0.82785e-1_f64 * t8571;
    (t8592, t8607)
}
