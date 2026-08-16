//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 413/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk413(t206: f64, t686: f64, t664: f64, t689: f64, t1719: f64, t226: f64, t1835: f64, t76: f64) -> (f64, f64, f64, f64, f64) {
    let t1937 = t686 * t206;
    let t1938 = t689 * t664;
    let t1939 = t1937 * t1938;
    let t1942 = t226 * t1719;
    let t1945 = t76 * t1835;
    (t1937, t1938, t1939, t1942, t1945)
}
