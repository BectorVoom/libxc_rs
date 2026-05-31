//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1074/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1074<F: Float>(t210: F, t17348: F, t1975: F, t252: F, t1978: F, t17402: F, t5870: F, t690: F, t1936: F, t239: F, t1939: F, t5801: F, t659: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t17444 = F::powf(t210, -F::cast_from(0.25e1_f64));
    let t17454 = F::cast_from(0.31310740740740740741e1_f64) * t17348;
    let t17455 = F::cast_from(280.0_f64) / F::cast_from(81.0_f64) * t17348;
    let t17473 = t1975 * t1975;
    let t17474 = F::cast_from(1.0_f64) / t17473;
    let t17475 = t252 * t17474;
    let t17477 = t1978 * t1978;
    let t17478 = F::cast_from(1.0_f64) / t17477;
    let t17487 = F::cast_from(0.16979925925925925926e1_f64) * t17402;
    let t17505 = F::cast_from(0.5356037037037037037e1_f64) * t17348;
    let t17514 = t690 * t5870;
    let t17517 = t1936 * t1936;
    let t17519 = t239 / t17517;
    let t17520 = t1939 * t1939;
    let t17521 = F::cast_from(1.0_f64) / t17520;
    let t17536 = t659 * t5801;
    (t17444, t17454, t17455, t17474, t17475, t17478, t17487, t17505, t17514, t17519, t17521, t17536)
}
