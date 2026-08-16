//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1075/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1075(t1873: f64, t22461: f64, t26103: f64, t6517: f64, t6534: f64, t30991: f64, t8601: f64, t2314: f64, t8326: f64, t5113: f64, t31029: f64, t31224: f64, t671: f64, t8446: f64) -> (f64, f64, f64) {
    let t31227 = t22461 * t1873;
    let t31229 = t26103 * t1873;
    let t31231 = t6517 * t6534;
    let t31233 = 2.0_f64 * t30991;
    let t31235 = 4.0_f64 * t8601 * t6534;
    let t31236 = t2314 * t8326;
    let t31237 = 2.0_f64 * t31236;
    let t31238 = t5113 * t8326;
    let t31239 = 2.0_f64 * t31238;
    let t31240 = 2.0_f64 * t31224 * t671 + t31029 + 4.0_f64 * t31227 + 4.0_f64 * t31229 + 4.0_f64 * t31231 + t31233 + t31235 + t31237 + t31239 + t8446;
    (t31237, t31239, t31240)
}
