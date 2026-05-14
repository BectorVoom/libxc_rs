//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 882/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk882<F: Float>(t11373: F, t11375: F, t11377: F, t11382: F, t11385: F, t11389: F, t11392: F, t11403: F, t11406: F, t11410: F, t11415: F, t11421: F, t11426: F, t11432: F, t11436: F, t11440: F, t11443: F, t11445: F, t11453: F, t11459: F) -> (F,) {
    let t12389 = -0.18115908419564701085e-6 * t11373 + 0.42206481990611010728e-7 * t11375 + 0.42206481990611010728e-7 * t11377 - 0.88397049170382309322e-8 * t11382 - 0.13259557375557346398e-6 * t11385 + 0.99044544404633838509e-5 * t11389 + 0.11594181388521408695e-4 * t11392 - 0.28680385873582628043e-8 * t11403 + 0.80966145833333333339e-4 * t11406 + 0.16867947048611111113e-5 * t11410 - 0.14759453667534722224e-5 * t11415 - 0.94685814672924837676e-4 * t11421 - 0.4048307291666666667e-3 * t11426 + 0.61551119569641057312e-8 * t11432 - 0.43440462632258606772e-4 * t11436 - 0.43440462632258606772e-4 * t11440 - 0.11372686522837130914e-5 * t11443 - 0.12817572129705434851e-5 * t11445 + 0.19666550313313802087e-7 * t11453 + 0.20220636637604418766e-5 * t11459;
    (t12389,)
}
