//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 857/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk857(t595: f64, t7006: f64, t1669: f64, t2799: f64, t2461: f64, t585: f64, t159: f64, t617: f64, t1655: f64, t2774: f64, t5451: f64, t5454: f64, t5459: f64, t5463: f64, t5467: f64, t5470: f64, t5903: f64, t598: f64, t951: f64) -> f64 {
    let t7768 = t595 * t7006;
    let t7776 = t2799 * t1669;
    let t7778 = t2461 * t585;
    let t7779 = t159 * t7778;
    let t7781 = 0.16936279733333333333e-2_f64 * t7779 * t617;
    let t7782 = -0.675260332e-1_f64 * t7768 * t598 - 0.1350520664e0_f64 * t2774 * t1655 - 0.675260332e-1_f64 * t951 * t5903 + t5451 + t5454 - t5459 + t5463 + t5467 + 0.84681398666666666666e-3_f64 * t5470 - 0.11290853155555555555e-2_f64 * t7776 + t7781;
    t7782
}
