//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1102/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1102(t3009: f64, t5195: f64, t1014: f64, t13335: f64, t1038: f64, t141: f64, t5065: f64, t664: f64) -> (f64, f64, f64, f64) {
    let t15232 = 0.5848223622634646207e0_f64 * t3009 * t5195;
    let t15235 = t1014 * t13335;
    let t15236 = t1038 * t15235;
    let t15237 = t141 * t15236;
    let t15239 = t664 * t5065;
    (t15232, t15235, t15237, t15239)
}
