//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 467/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk467(t265: f64, t394: f64, t1052: f64, t1920: f64, t1923: f64, t1946: f64, t1956: f64, t388: f64, t1914: f64, t202: f64, t193: f64, t870: f64, t1070: f64, t336: f64) -> (f64, f64, f64, f64) {
    let t395 = t265 < t394;
    let t1958 = 0.82246703342411321825e-2_f64 * t1920 * t1923 + t1946 * t388 - t1052 * t1956;
    let t1962 = t202 * t1914;
    let t1964 = t193 * t1962 * t870;
    let t1965 = piecewise3(t395, t193 * t336 * t1958 * t1070, t1964);
    (t1958, t1962, t1964, t1965)
}
