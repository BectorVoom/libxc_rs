//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 822/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk822(t2004: f64, t9090: f64, t2007: f64, t1987: f64, t1990: f64, t1173: f64, t674: f64, t9085: f64, t2868: f64, t7779: f64, t2186: f64, t8597: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40349 = t9090 * t2004;
    let t40350 = 0.19863479950205658386e-4_f64 * t40349;
    let t40351 = t9090 * t2007;
    let t40354 = t9090 * t1987;
    let t40356 = t9090 * t1990;
    let t40357 = 0.19863479950205658386e-4_f64 * t40356;
    let t40359 = t9085 * t1173 * t674;
    let t40458 = t2868 * t7779;
    let t40459 = 0.79828278012425390426e-1_f64 * t40458;
    let t40479 = t2186 * t8597;
    (t40350, t40351, t40354, t40357, t40359, t40459, t40479)
}
