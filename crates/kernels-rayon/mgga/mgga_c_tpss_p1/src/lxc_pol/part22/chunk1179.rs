//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1179/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1179(t4341: f64, t645: f64, t13220: f64, t485: f64, t2105: f64, t94: f64, t1600: f64, t1168: f64, t13131: f64, t13133: f64, t13136: f64, t13223: f64, t1339: f64, t1663: f64, t2056: f64, t2106: f64, t3174: f64, t3493: f64, t3499: f64, t3502: f64, t3538: f64, t3542: f64, t4541: f64, t488: f64, t544: f64, t626: f64, t646: f64) -> (f64, f64, f64, f64, f64) {
    let t13225 = t4341 * t645;
    let t13228 = t485 * t13220;
    let t13235 = t94 * t2105;
    let t13244 = t1600 * t2105;
    let t13251 = 2.0_f64 * t1168 * t4541 + t13131 * t488 - 4.0_f64 * t13133 * t646 - 2.0_f64 * t13136 * t485 + t13223 * t544 - 4.0_f64 * t13225 * t626 - 2.0_f64 * t13228 * t626 - 2.0_f64 * t13235 * t1339 - 2.0_f64 * t13244 * t626 + t1663 * t3174 - 4.0_f64 * t2056 * t3538 - 4.0_f64 * t2056 * t3542 - 2.0_f64 * t2106 * t3493 - 4.0_f64 * t3499 * t3502 - 4.0_f64 * t3499 * t3538;
    (t13225, t13228, t13235, t13244, t13251)
}
