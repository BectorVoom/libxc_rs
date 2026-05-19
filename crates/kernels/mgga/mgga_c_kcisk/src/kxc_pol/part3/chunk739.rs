//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 739/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk739<F: Float>(t11444: F, t706: F, t11396: F, t11400: F, t11405: F, t11409: F, t11414: F, t11420: F, t11423: F, t11427: F, t11431: F, t11435: F, t11440: F, t1421: F, t456: F) -> F {
    let t11445 = t706 * t11444;
    let t11448 = -F::new(0.59133867e-2) * t456 * t11396 - F::new(0.59133867e-2) * t11400 * t11405 + F::new(0.59133867e-2) * t1421 * t11409 + F::cast_from(0.887008005e-2_f64) * t1421 * t11414 + F::cast_from(0.29201909629629629629e-2_f64) * t1421 * t11420 - F::cast_from(0.26281718666666666667e-2_f64) * t11423 - F::cast_from(0.4435040025e-2_f64) * t1421 * t11427 - F::cast_from(0.4435040025e-2_f64) * t1421 * t11431 + F::cast_from(0.65704296666666666667e-3_f64) * t1421 * t11435 - F::cast_from(0.22175200125e-2_f64) * t1421 * t11440 - F::cast_from(0.36958666875e-3_f64) * t456 * t11445;
    t11448
}
