//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 414/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk414(t1691: f64, t225: f64, t704: f64, t61: f64, t1376: f64, t76: f64) -> (f64, f64, f64) {
    let t1693 = t704 * t1691 * t225;
    let t1695 = 0.1301229756036208781e0_f64 * t61 * t1693;
    let t1696 = t1376 * t76;
    (t1693, t1695, t1696)
}
