//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 391/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk391(t1691: f64, t225: f64, t704: f64, t61: f64, t732: f64, t745: f64, t1419: f64, t230: f64, t1422: f64, t717: f64, t720: f64, t424: f64, t697: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1693 = t704 * t1691 * t225;
    let t1695 = 0.1301229756036208781e0_f64 * t61 * t1693;
    let t1699 = t732 * t745;
    let t1702 = 12.0_f64 * t1419 * t230;
    let t1704 = 32.0_f64 * t1422 * t230;
    let t1706 = t717 * t1691;
    let t1707 = t1706 * t720;
    let t1709 = 0.19263893255070628431e1_f64 * t61 * t1707;
    let t1710 = t424 * t697;
    (t1693, t1695, t1699, t1702, t1704, t1707, t1709, t1710)
}
