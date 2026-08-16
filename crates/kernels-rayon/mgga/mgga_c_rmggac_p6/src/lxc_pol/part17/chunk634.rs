//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 634/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk634(t884: f64, t8866: f64, t2405: f64, t302: f64, t72: f64, t2298: f64, t4601: f64, t2025: f64, t5928: f64, t1664: f64, t668: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8867 = t884 * t8866;
    let t8869 = t302 * t2405;
    let t8870 = t72 * t8869;
    let t8872 = t4601 * t2298;
    let t8874 = t5928 * t2025;
    let t8876 = t1664 * t668;
    (t8867, t8869, t8870, t8872, t8874, t8876)
}
