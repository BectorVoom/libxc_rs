//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 328/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk328(t1484: f64, t1490: f64, t1494: f64, t1498: f64, t1502: f64, t1507: f64, t1514: f64, t1518: f64) -> f64 {
    let t1620 = 0.9375e-1_f64 * t1484 - 0.9375e-1_f64 * t1490 - 0.25e0_f64 * t1494 + 0.625e-1_f64 * t1498 - 0.101171875e-1_f64 * t1502 + 0.101171875e-1_f64 * t1507 + 0.53958333333333333333e-1_f64 * t1514 - 0.13489583333333333333e-1_f64 * t1518;
    t1620
}
