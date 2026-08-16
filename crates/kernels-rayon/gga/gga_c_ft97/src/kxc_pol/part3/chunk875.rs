//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 875/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk875(t15756: f64, t3621: f64, t2266: f64, t4462: f64, t643: f64, t15768: f64, t15763: f64, t3613: f64, t4454: f64, t8654: f64, t12143: f64, t17549: f64, t17552: f64, t17554: f64, t17556: f64, t17560: f64, t17564: f64, t17569: f64, t17573: f64, t17577: f64, t17583: f64, t17586: f64, t17590: f64, t17593: f64, t2265: f64, t631: f64, t8641: f64, t8719: f64) -> f64 {
    let t17595 = t3621 * t15756;
    let t17599 = t2266 * t4462 * t643;
    let t17602 = t3621 * t15768;
    let t17605 = t3613 * t15763;
    let t17609 = t8654 * t4454 * t643;
    let t17612 = -3.0_f64 * t631 * t17549 + 2.0_f64 / 9.0_f64 * t17552 - t17554 / 9.0_f64 - t17556 / 27.0_f64 - 3.0_f64 / 2.0_f64 * t631 * t17560 + t631 * t17564 / 6.0_f64 + 6.0_f64 * t631 * t17569 + 5.0_f64 / 27.0_f64 * t8641 + 4.0_f64 / 9.0_f64 * t17573 - t2265 * t17577 / 3.0_f64 + 5.0_f64 / 9.0_f64 * t8719 + t2265 * t17583 - 4.0_f64 / 3.0_f64 * t12143 * t17586 + 2.0_f64 / 3.0_f64 * t2265 * t17590 + t2265 * t17593 - 4.0_f64 / 3.0_f64 * t12143 * t17595 - t2265 * t17599 / 3.0_f64 - t2265 * t17602 / 3.0_f64 + t2265 * t17605 / 18.0_f64 - t2265 * t17609 / 9.0_f64;
    t17612
}
