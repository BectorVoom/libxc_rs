//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1127/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1127(t3109: f64, t4630: f64, t4650: f64, t884: f64, t3071: f64, t10436: f64, t10441: f64, t10449: f64, t10455: f64, t10460: f64, t10490: f64, t10496: f64, t10504: f64, t10511: f64, t10517: f64, t10863: f64, t10866: f64, t10871: f64, t1618: f64, t1622: f64, t3048: f64, t3070: f64, t4636: f64) -> f64 {
    let t14059 = t3109 * t4630 / 432.0_f64;
    let t14068 = t4650 * t884;
    let t14069 = t3071 * t14068;
    let t14074 = -t10436 / 6912.0_f64 - t10441 / 432.0_f64 + 19.0_f64 / 2592.0_f64 * t10449 + t10455 / 6912.0_f64 + 5.0_f64 / 20736.0_f64 * t10460 + 19.0_f64 / 1728.0_f64 * t10517 * t1618 - t14059 - t10863 * t1622 / 432.0_f64 - t3048 * t4636 / 432.0_f64 - t10490 / 3456.0_f64 - t10496 / 432.0_f64 + t10504 / 2304.0_f64 - t10511 / 6912.0_f64 + t3070 * t14069 / 2304.0_f64 + t10866 / 3456.0_f64 - t10871 / 10368.0_f64;
    t14074
}
