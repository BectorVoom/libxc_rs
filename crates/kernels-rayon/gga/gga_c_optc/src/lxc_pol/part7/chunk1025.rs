//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1025/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1025(t43: f64, t40: f64, t591: f64, t6735: f64, t1871: f64, t1906: f64, t768: f64, t97: f64, t1884: f64, t1885: f64, t1891: f64, t22015: f64, t22021: f64, t22028: f64, t47: f64, t6541: f64, t6713: f64, t6716: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t44 = t43 <= zeta_threshold;
    let t22302 = t40 * t6735 * t591;
    let t22303 = 4.0_f64 * t22302;
    let t22305 = t40 * t1906 * t1871;
    let t22306 = 6.0_f64 * t22305;
    let t22308 = 1.0_f64 / t97 / t768;
    let t22321 = piecewise3(t44, 0.0_f64, 40.0_f64 / 81.0_f64 * t22308 * t22015 - 16.0_f64 / 9.0_f64 * t6713 * t1885 * t1891 + 4.0_f64 / 3.0_f64 * t1884 * t22021 + 16.0_f64 / 9.0_f64 * t6716 * t6541 + 4.0_f64 / 3.0_f64 * t47 * t22028);
    (t22303, t22306, t22321)
}
