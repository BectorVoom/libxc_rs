//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1001/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1001<F: Float>(t11373: F, t11375: F, t11377: F, t11382: F, t11385: F, t11389: F, t11392: F, t11403: F, t11406: F, t11410: F, t11415: F, t11421: F, t11426: F, t11432: F, t11436: F, t11440: F, t11443: F, t11445: F, t11453: F, t11459: F) -> F {
    let t12389 = -F::cast_from(0.18115908419564701085e-6_f64) * t11373 + F::cast_from(0.42206481990611010728e-7_f64) * t11375 + F::cast_from(0.42206481990611010728e-7_f64) * t11377 - F::cast_from(0.88397049170382309322e-8_f64) * t11382 - F::cast_from(0.13259557375557346398e-6_f64) * t11385 + F::cast_from(0.99044544404633838509e-5_f64) * t11389 + F::cast_from(0.11594181388521408695e-4_f64) * t11392 - F::cast_from(0.28680385873582628043e-8_f64) * t11403 + F::cast_from(0.80966145833333333339e-4_f64) * t11406 + F::cast_from(0.16867947048611111113e-5_f64) * t11410 - F::cast_from(0.14759453667534722224e-5_f64) * t11415 - F::cast_from(0.94685814672924837676e-4_f64) * t11421 - F::cast_from(0.4048307291666666667e-3_f64) * t11426 + F::cast_from(0.61551119569641057312e-8_f64) * t11432 - F::cast_from(0.43440462632258606772e-4_f64) * t11436 - F::cast_from(0.43440462632258606772e-4_f64) * t11440 - F::cast_from(0.11372686522837130914e-5_f64) * t11443 - F::cast_from(0.12817572129705434851e-5_f64) * t11445 + F::cast_from(0.19666550313313802087e-7_f64) * t11453 + F::cast_from(0.20220636637604418766e-5_f64) * t11459;
    t12389
}
