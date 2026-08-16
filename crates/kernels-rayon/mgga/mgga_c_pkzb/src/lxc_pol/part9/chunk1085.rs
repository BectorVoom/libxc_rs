//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1085/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1085(t210: f64, t17348: f64, t1975: f64, t252: f64, t1978: f64, t17402: f64, t5870: f64, t690: f64, t1936: f64, t239: f64, t1939: f64, t5801: f64, t659: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t17444 = f64::powf(t210, -0.25e1_f64);
    let t17454 = 0.31310740740740740741e1_f64 * t17348;
    let t17455 = 280.0_f64 / 81.0_f64 * t17348;
    let t17473 = t1975 * t1975;
    let t17474 = 1.0_f64 / t17473;
    let t17475 = t252 * t17474;
    let t17477 = t1978 * t1978;
    let t17478 = 1.0_f64 / t17477;
    let t17487 = 0.16979925925925925926e1_f64 * t17402;
    let t17505 = 0.5356037037037037037e1_f64 * t17348;
    let t17514 = t690 * t5870;
    let t17517 = t1936 * t1936;
    let t17519 = t239 / t17517;
    let t17520 = t1939 * t1939;
    let t17521 = 1.0_f64 / t17520;
    let t17536 = t659 * t5801;
    (t17444, t17454, t17455, t17474, t17475, t17478, t17487, t17505, t17514, t17519, t17521, t17536)
}
