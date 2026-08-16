//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 970/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk970(t2604: f64, t8997: f64, t2024: f64, t5249: f64, t1356: f64, t27044: f64, t27120: f64, t29892: f64, t35777: f64, t35782: f64, t35787: f64, t40554: f64, t40556: f64, t40559: f64, t40561: f64, t40563: f64, t40565: f64, t40567: f64, t40568: f64, t40573: f64, t40575: f64, t739: f64, t7703: f64, t884: f64) -> (f64, f64) {
    let t40578 = t2604 * t8997;
    let t40579 = 0.79828278012425390426e-1_f64 * t40578;
    let t40589 = t2024 * t5249;
    let t40592 = -t35777 - t35782 + t35787 + 0.53205749866622299248e-5_f64 * t40554 - 0.12769379967989351819e-4_f64 * t40556 - t40559 + t40561 - t40563 - t40565 + t40567 + 0.1064114997332445985e-4_f64 * t40568 + 0.1064114997332445985e-4_f64 * t40573 + 0.11974241701863808564e0_f64 * t884 * t40575 - t40579 + 0.23948483403727617128e0_f64 * t739 * t2024 * t29892 - 0.23948483403727617128e0_f64 * t884 * t2024 * t27044 - 0.23948483403727617128e0_f64 * t1356 * t7703 * t27120 + 0.39914139006212695214e-1_f64 * t1356 * t40589;
    (t40589, t40592)
}
