//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1279/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1279(t30315: f64, t574: f64, t1268: f64, t12725: f64, t19456: f64, t2200: f64, t2202: f64, t2314: f64, t26114: f64, t26117: f64, t26179: f64, t4028: f64, t5113: f64, t7458: f64, t7676: f64, t8176: f64, t8190: f64, t8194: f64, t8278: f64, t8280: f64) -> (f64, f64) {
    let t30330 = t30315 * t574;
    let t30347 = t1268 * t30330 - t12725 * t2200 - t19456 * t2200 + t19456 * t2202 - t2200 * t26114 - t2200 * t26179 + t2202 * t26117 + t2314 * t8278 + t2314 * t8280 - t4028 * t8176 - t4028 * t8190 + t4028 * t8194 + t5113 * t8278 + t5113 * t8280 - t7458 * t8176 + t7676 * t8194;
    (t30330, t30347)
}
