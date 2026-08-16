//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1011/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1011(t42944: f64, t5241: f64, t5640: f64, t590: f64, t33289: f64, t7810: f64, t9889: f64, t2028: f64, t3038: f64, t787: f64, t9636: f64, t13055: f64, t28073: f64) -> (f64, f64, f64, f64) {
    let t43361 = 0.13803453343411469884e2_f64 * t5640 * t5241 * t42944 * t590;
    let t43363 = t7810 * t33289 * t9889;
    let t43364 = 0.19171462976960374838e1_f64 * t43363;
    let t43368 = 0.39722766613167140743e-1_f64 * t787 * t9636 * t3038 * t2028;
    let t43370 = t28073 * t13055;
    (t43361, t43364, t43368, t43370)
}
