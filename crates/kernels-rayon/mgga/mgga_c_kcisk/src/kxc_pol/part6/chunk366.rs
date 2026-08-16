//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 366/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk366(t2260: f64, t2264: f64, t2268: f64, t2272: f64, t2276: f64, t2280: f64) -> f64 {
    let t2347 = 0.9375e-1_f64 * t2260 - 0.9375e-1_f64 * t2264 + 0.625e-1_f64 * t2268 - 0.101171875e-1_f64 * t2272 + 0.101171875e-1_f64 * t2276 - 0.13489583333333333333e-1_f64 * t2280;
    t2347
}
