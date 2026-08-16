//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 418/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk418(t1716: f64, t586: f64, t1400: f64, t1403: f64, t1405: f64, t1408: f64, t1714: f64) -> (f64, f64) {
    let t1717 = t586 * t1716;
    let t1719 = -0.28769444444444444445e0_f64 * t1714 + 0.23015555555555555556e1_f64 * t1717 + t1400 + t1403 + t1405 + t1408;
    (t1717, t1719)
}
