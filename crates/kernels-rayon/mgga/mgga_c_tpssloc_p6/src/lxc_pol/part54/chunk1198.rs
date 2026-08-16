//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1198/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1198(t32368: f64, t32371: f64, t32378: f64, t32390: f64, t3: f64, t112: f64, t8843: f64, t2039: f64, t24969: f64, t24972: f64, t31284: f64, t31287: f64, t31799: f64, t31801: f64, t31803: f64, t31811: f64, t31813: f64, t31816: f64, t31819: f64, t577: f64, t671: f64, t7056: f64, t7235: f64, t7423: f64, t8508: f64) -> (f64, f64, f64, f64) {
    let t32392 = t32368 + t32371 + t32378 + t32390;
    let t32393 = t3 * t32392;
    let t32406 = t8843 * t112;
    let t32415 = 0.45e1_f64 * t32392 * t577 + 0.135e2_f64 * t32406 * t671 + 0.135e2_f64 * t24969 * t2039 + 27.0_f64 * t24972 * t7235 + 0.135e2_f64 * t7423 * t7056 + t31799 + t31801 + t31803 + t31811 + t31813 + t31816 + t31819 + t31284 + t31287 + t8508;
    (t32392, t32393, t32406, t32415)
}
