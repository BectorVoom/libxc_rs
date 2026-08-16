//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 951/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk951(t10879: f64, t3305: f64, t2172: f64, t261: f64, t3304: f64, t2190: f64, t3299: f64, t2197: f64, t7614: f64, t2218: f64, t503: f64, t505: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10880 = t10879 * t3305;
    let t10882 = t261 * t2172;
    let t10883 = t3304 * t10882;
    let t10885 = t261 * t2190;
    let t10886 = t3299 * t10885;
    let t10887 = 0.23115257973478049502e0_f64 * t10886;
    let t10888 = t261 * t2197;
    let t10889 = t7614 * t10888;
    let t10891 = t261 * t2218;
    let t10892 = t3304 * t10891;
    let t10893 = 0.69345773920434148506e0_f64 * t10892;
    let t10894 = t503 * t505;
    (t10880, t10882, t10883, t10885, t10886, t10887, t10888, t10889, t10891, t10892, t10893, t10894)
}
