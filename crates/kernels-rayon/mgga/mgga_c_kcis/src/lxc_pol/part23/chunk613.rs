//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 613/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk613(t1555: f64, t5897: f64, t2069: f64, t4184: f64, t4189: f64, t4291: f64, t576: f64, t251: f64, t4301: f64, t5875: f64, t492: f64, t570: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5898 = t5897 * t1555;
    let t5899 = t4184 * t2069;
    let t5900 = t2069 * t1555;
    let t5902 = 2.0_f64 * t4189 * t5900;
    let t5903 = t576 * t4291;
    let t5904 = t251 * t4301;
    let t5905 = t5904 * t5875;
    let t5906 = t5903 * t5905;
    let t5908 = t570 * t492;
    (t5898, t5899, t5900, t5902, t5903, t5904, t5905, t5906, t5908)
}
