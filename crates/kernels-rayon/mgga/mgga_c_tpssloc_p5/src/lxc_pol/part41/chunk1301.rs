//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1301/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1301(t1268: f64, t12725: f64, t19456: f64, t2200: f64, t26114: f64, t26117: f64, t28002: f64, t28030: f64, t30269: f64, t30316: f64, t30321: f64, t30330: f64, t30565: f64, t4028: f64, t5113: f64, t5361: f64, t6468: f64, t7458: f64, t7676: f64, t8176: f64, t8189: f64, t8190: f64, t8260: f64, t8273: f64, t8278: f64, t96709: f64, t97933: f64) -> f64 {
    let t112049 = 4.0_f64 * t1268 * t5361 * t8273 + 2.0_f64 * t1268 * t6468 * t8189 - 4.0_f64 * t12725 * t8260 + 4.0_f64 * t19456 * t8278 - 2.0_f64 * t2200 * t96709 - 2.0_f64 * t2200 * t97933 - 4.0_f64 * t26114 * t8260 + 4.0_f64 * t26114 * t8278 + 4.0_f64 * t26117 * t8278 - 4.0_f64 * t28002 * t8190 - 2.0_f64 * t28030 * t8176 - 2.0_f64 * t28030 * t8190 + 4.0_f64 * t30269 * t4028 + 4.0_f64 * t30269 * t7676 - 4.0_f64 * t30316 * t4028 - 4.0_f64 * t30316 * t7458 + 4.0_f64 * t30321 * t7676 + 4.0_f64 * t30330 * t4028 + 4.0_f64 * t30330 * t7676 + 2.0_f64 * t30565 * t5113;
    t112049
}
