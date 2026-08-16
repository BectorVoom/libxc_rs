//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 640/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk640(t139: f64, t8894: f64, t548: f64, t135: f64, t2059: f64, t554: f64, t8078: f64, t8074: f64, t8082: f64, t8086: f64, t8091: f64, t8094: f64, t8096: f64, t8099: f64, t8104: f64, t8107: f64, t8110: f64, t8113: f64, t8116: f64, t8123: f64, t8127: f64, t8131: f64, t8133: f64, t8135: f64, t8137: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8895 = t8894 * t139;
    let t8906 = t548 * t548;
    let t8907 = 1.0_f64 / t8906;
    let t8908 = t135 * t8907;
    let t8909 = t2059 * t554;
    let t8914 = 0.18521666970164609055e-1_f64 * t8078;
    let t8932 = 0.88904001456790123462e-1_f64 * t8074 + t8914 - 0.22818693707242798355e1_f64 * t8082 + 0.48897200801234567904e0_f64 * t8086 + 0.10001700163888888889e0_f64 * t8091 - 0.10001700163888888889e0_f64 * t8094 + 0.26671200437037037038e0_f64 * t8096 - 0.33339000546296296299e-1_f64 * t8099 - 0.13335600218518518519e0_f64 * t8104 + 0.66678001092592592595e-1_f64 * t8107 - 0.11113000182098765433e-1_f64 * t8110 + 0.16669500273148148149e-1_f64 * t8113 + 0.22226000364197530866e-1_f64 * t8116 + 0.51860667516460905352e-1_f64 * t8123 + 0.16669500273148148149e-1_f64 * t8127 + 0.48897200801234567904e0_f64 * t8131 - 0.88904001456790123462e-1_f64 * t8133 - 0.13335600218518518519e0_f64 * t8135 - 0.17780800291358024692e0_f64 * t8137;
    (t8895, t8906, t8907, t8908, t8909, t8932)
}
