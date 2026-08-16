//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1298/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1298(t6077: f64, t62306: f64, t18646: f64, t6080: f64, t65442: f64, t65444: f64, t116: f64, t20287: f64, t20217: f64, t508: f64, t20319: f64, t1665: f64, t5960: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t67510 = t62306 * t6077;
    let t67512 = t6080 * t18646;
    let t67532 = 8.0_f64 / 3.0_f64 * t65442;
    let t67533 = 4.0_f64 / 3.0_f64 * t65444;
    let t67541 = t20287 * t116;
    let t67782 = t508 * t20217;
    let t67816 = t116 * t20319;
    let t67849 = 2.0_f64 * t1665 * t5960;
    (t67510, t67512, t67532, t67533, t67541, t67782, t67816, t67849)
}
