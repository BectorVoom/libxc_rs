//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1265/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1265(t3579: f64, t38723: f64, t3275: f64, t3472: f64, t39178: f64, t11325: f64, t11518: f64, t3262: f64, t11189: f64, t40289: f64, t3465: f64, t40667: f64) -> (f64, f64, f64, f64, f64) {
    let t42277 = t3579 * t38723 / 2.0_f64;
    let t42281 = 5.0_f64 / 16.0_f64 * t3275 * t3472 * t39178;
    let t42284 = 15.0_f64 / 8.0_f64 * t3262 * t11325 * t11518;
    let t42287 = 45.0_f64 / 64.0_f64 * t3275 * t11189 * t40289;
    let t42290 = 3.0_f64 / 2.0_f64 * t3275 * t3465 * t40667;
    (t42277, t42281, t42284, t42287, t42290)
}
