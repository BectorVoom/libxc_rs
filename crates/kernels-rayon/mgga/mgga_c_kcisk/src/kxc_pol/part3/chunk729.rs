//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 729/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk729(t11287: f64, t11290: f64, t11294: f64, t11298: f64, t11302: f64, t11306: f64, t11309: f64, t11314: f64, t11316: f64, t11318: f64, t11320: f64, t1421: f64) -> f64 {
    let t11322 = -0.32852148333333333333e-2_f64 * t1421 * t11287 + 0.32852148333333333333e-2_f64 * t1421 * t11290 + 0.295669335e-2_f64 * t1421 * t11294 + 0.295669335e-2_f64 * t1421 * t11298 - 0.19711289e-2_f64 * t1421 * t11302 - 0.19711289e-2_f64 * t1421 * t11306 - 0.39422577999999999999e-2_f64 * t1421 * t11309 - 0.43802864444444444445e-3_f64 * t11314 + 0.13140859333333333334e-2_f64 * t11316 + 0.21901432222222222222e-2_f64 * t11318 - 0.59133867e-2_f64 * t11320;
    t11322
}
