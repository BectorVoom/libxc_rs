//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1008/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1008<F: Float>(t1878: F, t1889: F, t218: F, t5568: F, t675: F, t5572: F, t16194: F, t213: F, t778: F, t210: F, t17348: F, t1975: F, t252: F, t1978: F, t17402: F, t5870: F, t690: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t17411 = t218 * t1878 * t1889;
    let t17414 = t218 * t675 * t5568;
    let t17417 = t218 * t675 * t5572;
    let t17432 = 1.0 / t213 / t16194 / t778 / 96.0;
    let t17444 = f64::powf(t210, -0.25e1);
    let t17454 = 0.31310740740740740741e1 * t17348;
    let t17455 = 280.0 / 81.0 * t17348;
    let t17473 = t1975 * t1975;
    let t17474 = 1.0 / t17473;
    let t17475 = t252 * t17474;
    let t17477 = t1978 * t1978;
    let t17478 = 1.0 / t17477;
    let t17487 = 0.16979925925925925926e1 * t17402;
    let t17505 = 0.5356037037037037037e1 * t17348;
    let t17514 = t690 * t5870;
    (t17411, t17414, t17417, t17432, t17444, t17454, t17455, t17474, t17475, t17478, t17487, t17505, t17514)
}
