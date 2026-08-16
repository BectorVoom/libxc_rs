//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1076/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1076(t13384: f64, t829: f64, t13176: f64, t13336: f64, t13429: f64, t13431: f64, t13434: f64, t13448: f64, t13450: f64, t13453: f64, t1499: f64, t1523: f64, t1525: f64, t226: f64, t255: f64, t2613: f64, t2617: f64, t2738: f64, t2740: f64, t4162: f64, t4166: f64, t4283: f64, t4286: f64, t4288: f64, t4291: f64, t4298: f64, t808: f64, t812: f64, t861: f64, t863: f64, t9612: f64) -> f64 {
    let t13456 = t13384 * t829;
    let t13459 = -2.0_f64 * t13176 * t861 + t13336 * t255 - t13429 * t812 - t13431 * t812 - 2.0_f64 * t13434 * t812 + t13448 * t226 - t13450 * t4291 + 4.0_f64 * t13453 * t4283 - 2.0_f64 * t13456 * t4291 + t1499 * t2740 - t1523 * t9612 + t1525 * t2613 - 2.0_f64 * t2617 * t4286 - 2.0_f64 * t2617 * t4288 - t2738 * t4166 + 2.0_f64 * t4162 * t863 + 2.0_f64 * t4298 * t808;
    t13459
}
