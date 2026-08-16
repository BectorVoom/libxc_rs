//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1413/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1413(t18367: f64, t449: f64, t446: f64, t1659: f64, t2794: f64, t13045: f64, t13048: f64, t13050: f64, t13053: f64, t13055: f64, t13057: f64, t13060: f64, t13094: f64, t13096: f64, t15795: f64, t15798: f64, t8524: f64, t9272: f64, t9313: f64, t9315: f64) -> f64 {
    let t18368 = t449 * t18367;
    let t18369 = t446 * t18368;
    let t18371 = t2794 * t1659;
    let t18373 = t8524 + t9315 + t13045 / 8.0_f64 - t9313 - t13048 / 16.0_f64 - t13050 / 8.0_f64 - t13053 / 8.0_f64 + t13055 / 8.0_f64 + t13057 / 8.0_f64 - t13060 / 8.0_f64 - t9272 + t13094 + t13096 - t15795 / 16.0_f64 - t15798 / 16.0_f64 - t18369 / 16.0_f64 - t18371 / 8.0_f64;
    t18373
}
