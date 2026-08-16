//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 751/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk751(t169: f64, t449: f64, t4504: f64, t446: f64, t1646: f64, t2629: f64, t167: f64, t171: f64, t740: f64, t829: f64, t1650: f64, t2641: f64, t176: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t170 = t169 <= zeta_threshold;
    let t4505 = t449 * t4504;
    let t4506 = t446 * t4505;
    let t4507 = t4506 / 16.0_f64;
    let t4510 = t2629 * t1646;
    let t4513 = t171 * t167;
    let t4517 = piecewise3(t170, 0.0_f64, 4.0_f64 / 9.0_f64 * t4510 * t829 + 8.0_f64 / 3.0_f64 * t4513 * t740);
    let t4518 = t2641 * t1650;
    let t4521 = t176 * t167;
    (t4505, t4507, t4510, t4513, t4517, t4518, t4521)
}
