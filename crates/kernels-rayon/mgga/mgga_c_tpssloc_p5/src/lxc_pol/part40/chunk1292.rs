//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1292/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1292(t110082: f64, t110097: f64, t110102: f64, t110103: f64, t110503: f64, t110506: f64, t110510: f64, t110520: f64, t110521: f64, t110601: f64, t111326: f64, t111331: f64, t1444: f64, t19492: f64, t29903: f64, t29922: f64, t29926: f64, t30164: f64, t30175: f64, t30410: f64, t4067: f64, t5396: f64, t5464: f64, t5468: f64, t5488: f64, t659: f64, t666: f64, t8128: f64, t8129: f64, t8137: f64, t8138: f64, t96715: f64, t96718: f64, t96723: f64) -> f64 {
    let t111379 = 44.0_f64 / 9.0_f64 * t110503 + t110506 - 110.0_f64 / 27.0_f64 * t110510 - 20.0_f64 / 9.0_f64 * t111326 - 25.0_f64 / 18.0_f64 * t8128 * t29922 * t30410 + 5.0_f64 / 6.0_f64 * t110601 * t8138 * t111331 - 5.0_f64 / 18.0_f64 * t30175 * t29926 * t19492 + t110102 + 55.0_f64 / 27.0_f64 * t110103 + 3.0_f64 * t110082 * t8129 * t96715 - 5.0_f64 / 4.0_f64 * t29903 * t8138 * t5464 * t659 - 3.0_f64 / 2.0_f64 * t29903 * t8129 * t96718 + 5.0_f64 / 6.0_f64 * t8128 * t8138 * t4067 * t1444 - 3.0_f64 / 4.0_f64 * t29903 * t8129 * t96723 + 5.0_f64 / 12.0_f64 * t8128 * t8138 * t5488 * t659 + 5.0_f64 / 18.0_f64 * t8128 * t29926 * t5468 * t666 + 5.0_f64 / 108.0_f64 * t8137 * t110097 * t5468 * t659 + 5.0_f64 / 12.0_f64 * t8128 * t8138 * t5396 * t666 - 5.0_f64 / 36.0_f64 * t8137 * t29926 * t5396 * t659 - 5.0_f64 / 2.0_f64 * t110520 * t110521 * t30164;
    t111379
}
