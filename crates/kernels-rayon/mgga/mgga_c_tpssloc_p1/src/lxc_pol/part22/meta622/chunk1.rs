//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2156/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2156(t3032: f64, t52434: f64, t3505: f64, t3514: f64, t11835: f64, t4889: f64, t1174: f64, t1725: f64, t2402: f64, t3506: f64, t4979: f64, t49850: f64) -> (f64, f64, f64, f64, f64) {
    let t53371 = t52434 * t3032;
    let t53372 = t53371 * t3505;
    let t53399 = t53371 * t3514;
    let t53433 = t4889 * t11835;
    let t53434 = t53433 / 162.0_f64;
    let t53440 = t1174 * t2402 * t1725;
    let t53452 = t3506 * t49850 * t4979;
    (t53372, t53399, t53434, t53440, t53452)
}
