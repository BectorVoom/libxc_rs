//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1263/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1263(t114: f64, t1338: f64, t6399: f64, t21027: f64, t5909: f64, t1799: f64, t5314: f64, t1830: f64, t4674: f64, t18690: f64, t21011: f64, t18622: f64, t19588: f64, t21185: f64, t21187: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t115 = 1.0_f64 < t114;
    let t21880 = t6399 * t1338;
    let t21883 = t5909 * t21027;
    let t21894 = t5314 * t1799;
    let t21897 = t1830 * t4674;
    let t21900 = t18690 * t21011;
    let t21907 = piecewise3(t115, 0.0_f64, t18622 + 4.0_f64 / 3.0_f64 * t19588 + t21185 / 2.0_f64 - t21187 / 4.0_f64);
    (t21880, t21883, t21894, t21897, t21900, t21907)
}
