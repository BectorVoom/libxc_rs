//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 970/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk970(t13335: f64, t36: f64, t70: f64, t4580: f64, t602: f64, t1306: f64, t3426: f64, t3432: f64, t1290: f64, t3462: f64, t1314: f64, t13321: f64, t13322: f64, t13325: f64, t13331: f64, t3427: f64, t4574: f64, t4581: f64, t4584: f64, t616: f64, t85: f64) -> (f64, f64) {
    let t13336 = t36 * t13335;
    let t13337 = t13336 * t70;
    let t13340 = t4580 * t602;
    let t13345 = t3426 * t1306;
    let t13348 = t3432 * t1306;
    let t13351 = t1290 * t3462;
    let t13358 = -t13321 * t13322 / 6.0_f64 - t13325 * t85 / 12.0_f64 - t4574 * t616 / 12.0_f64 - t13331 * t85 / 12.0_f64 - t13337 * t85 / 12.0_f64 - t13340 * t85 / 12.0_f64 - t4581 * t616 / 12.0_f64 - t13345 * t85 / 6.0_f64 - t13348 * t85 / 6.0_f64 - t13351 * t85 / 6.0_f64 - t4584 * t616 / 6.0_f64 - t3427 * t1314 / 6.0_f64;
    (t13336, t13358)
}
