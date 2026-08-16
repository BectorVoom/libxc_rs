//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 540/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk540(t322: f64, t1010: f64, t1276: f64, t2378: f64, t2924: f64, t2928: f64, t2938: f64, t321: f64, t819: f64) -> (f64, f64) {
    let t324 = 0.0_f64 < t322;
    let t2940 = -2.0_f64 * t2378 * t1010 + 2.0_f64 * t1276 * t2928 + t2924 * t321 - t819 * t2938;
    let t2941 = piecewise3(t324, 0.0_f64, t2940);
    (t2940, t2941)
}
