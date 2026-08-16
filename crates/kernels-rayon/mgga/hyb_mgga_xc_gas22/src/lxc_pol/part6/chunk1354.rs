//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1354/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1354(t10551: f64, t260: f64, t10671: f64, t10679: f64, t2307: f64, t2326: f64, t2330: f64, t29392: f64, t29394: f64, t29396: f64, t29398: f64, t29400: f64, t29402: f64, t29404: f64, t3430: f64, t4207: f64, t6759: f64, t856: f64, t858: f64, t8941: f64) -> f64 {
    let t29494 = t260 * t10551;
    let t29508 = -0.11696447245269292414e1_f64 * t29494 * t858 + 0.11696447245269292414e1_f64 * t10679 * t2326 - 0.5848223622634646207e0_f64 * t10679 * t2330 + 0.11696447245269292414e1_f64 * t6759 * t4207 + 0.11696447245269292414e1_f64 * t856 * t10671 * t2307 - 0.70178683471615754484e1_f64 * t3430 * t8941 - t29392 + t29394 - t29396 - t29398 + t29400 - t29402 + t29404;
    t29508
}
