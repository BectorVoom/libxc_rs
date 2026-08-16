//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1057/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1057(t39591: f64, t2265: f64, t5026: f64, t1685: f64, t2212: f64, t2231: f64, t2604: f64, t30221: f64, t35149: f64, t35152: f64, t35184: f64, t35188: f64, t39577: f64, t39584: f64, t39589: f64, t39595: f64, t39600: f64, t39605: f64, t39607: f64, t39609: f64, t5898: f64, t72: f64, t8264: f64, t884: f64, t9383: f64) -> f64 {
    let t43042 = 0.1489760996265424379e-3_f64 * t39591;
    let t43043 = t5026 * t2265;
    let t43059 = -0.23948483403727617128e0_f64 * t884 * t8264 * t5898 - 0.14897609962654243789e-3_f64 * t35149 - 0.85129199786595678799e-5_f64 * t39577 + 0.49658699875514145964e-4_f64 * t35152 + 0.2553875993597870364e-4_f64 * t39584 - 0.71827762319940103988e-4_f64 * t39589 - t43042 - 0.2363e1_f64 * t43043 - 0.11974241701863808564e0_f64 * t2604 * t9383 - 0.5987120850931904282e-1_f64 * t39595 + 2.0_f64 * t72 * t1685 * t2231 + 0.2553875993597870364e-4_f64 * t39600 + 0.85129199786595678799e-5_f64 * t39605 - 0.85129199786595678799e-5_f64 * t39607 + 0.79828278012425390428e-1_f64 * t30221 * t2212 - 0.1440846329149835838e-2_f64 * t39609 - 0.10909864661698136692e0_f64 * t35184 - 0.54549323308490683461e-1_f64 * t35188;
    t43059
}
