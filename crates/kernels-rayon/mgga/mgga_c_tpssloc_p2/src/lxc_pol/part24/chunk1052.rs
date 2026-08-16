//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1052/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1052(t25: f64, t28: f64, t11988: f64, t12061: f64, t12064: f64, t2249: f64, t514: f64, t9257: f64, t528: f64, t1081: f64, t3672: f64, t11122: f64, t12001: f64, t3231: f64, t517: f64, zeta_threshold: f64) -> (f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t29 = t28 <= zeta_threshold;
    let t12070 = piecewise3(t26, 0.0_f64, -8.0_f64 / 27.0_f64 * t12061 * t11988 + 4.0_f64 / 3.0_f64 * t12064 * t2249 + 4.0_f64 / 3.0_f64 * t514 * t9257);
    let t12072 = 1.0_f64 / t528 / t28;
    let t12075 = t3672 * t1081;
    let t12081 = piecewise3(t29, 0.0_f64, -8.0_f64 / 27.0_f64 * t12072 * t12001 + 4.0_f64 / 3.0_f64 * t12075 * t3231 + 4.0_f64 / 3.0_f64 * t517 * t11122);
    (t12070, t12081)
}
