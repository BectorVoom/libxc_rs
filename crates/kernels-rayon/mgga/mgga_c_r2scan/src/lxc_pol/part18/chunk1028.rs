//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1028/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1028(t19790: f64, t560: f64, t545: f64, t6534: f64, t1567: f64, t489: f64, t146: f64, t252: f64, t108: f64, t2214: f64, t10979: f64, t128: f64) -> (f64, f64, f64, f64, f64) {
    let t20146 = t19790 * t560;
    let t20298 = t545 * t6534;
    let t20303 = t489 * t1567;
    let t20305 = t146 * t20303 * t252;
    let t20407 = t2214 * t108;
    let t20421 = t10979 * t128;
    (t20146, t20298, t20305, t20407, t20421)
}
