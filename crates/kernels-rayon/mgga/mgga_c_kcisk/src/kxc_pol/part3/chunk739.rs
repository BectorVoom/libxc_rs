//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 739/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk739(t11444: f64, t706: f64, t11396: f64, t11400: f64, t11405: f64, t11409: f64, t11414: f64, t11420: f64, t11423: f64, t11427: f64, t11431: f64, t11435: f64, t11440: f64, t1421: f64, t456: f64) -> f64 {
    let t11445 = t706 * t11444;
    let t11448 = -0.59133867e-2_f64 * t456 * t11396 - 0.59133867e-2_f64 * t11400 * t11405 + 0.59133867e-2_f64 * t1421 * t11409 + 0.887008005e-2_f64 * t1421 * t11414 + 0.29201909629629629629e-2_f64 * t1421 * t11420 - 0.26281718666666666667e-2_f64 * t11423 - 0.4435040025e-2_f64 * t1421 * t11427 - 0.4435040025e-2_f64 * t1421 * t11431 + 0.65704296666666666667e-3_f64 * t1421 * t11435 - 0.22175200125e-2_f64 * t1421 * t11440 - 0.36958666875e-3_f64 * t456 * t11445;
    t11448
}
