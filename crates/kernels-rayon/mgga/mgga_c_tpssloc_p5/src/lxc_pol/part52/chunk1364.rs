//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1364/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1364(t1409: f64, t8513: f64, t8514: f64, t1433: f64, t1862: f64, t113875: f64, t645: f64, t4021: f64, t641: f64, t31691: f64, t4017: f64, t115903: f64, t119901: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t121050 = t8513 * t8514 * t1409;
    let t121053 = t1862 * t1433;
    let t121055 = t113875 * t121053 * t645;
    let t121074 = t8513 * t8514 * t4021;
    let t121079 = t641 * t1862;
    let t121081 = t8513 * t121079 * t1433;
    let t121087 = t8513 * t31691 * t4017;
    let t121099 = t115903 * t119901;
    (t121050, t121055, t121074, t121081, t121087, t121099)
}
