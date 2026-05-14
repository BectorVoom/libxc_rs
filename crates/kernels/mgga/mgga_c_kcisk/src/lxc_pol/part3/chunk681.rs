//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 681/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk681<F: Float>(t4624: F, t682: F, t1824: F, t7028: F, t4663: F, t708: F, t10664: F, t706: F, t11396: F, t11400: F, t11405: F, t11409: F, t11414: F, t11420: F, t11423: F, t11427: F, t11431: F, t11435: F, t1421: F, t456: F) -> (F, F, F) {
    let t11438 = t682 * t4624;
    let t11439 = t11438 * t1824;
    let t11440 = t7028 * t11439;
    let t11443 = t4663 * t708;
    let t11444 = t11443 * t10664;
    let t11445 = t706 * t11444;
    let t11448 = -0.59133867e-2 * t456 * t11396 - 0.59133867e-2 * t11400 * t11405 + 0.59133867e-2 * t1421 * t11409 + 0.887008005e-2 * t1421 * t11414 + 0.29201909629629629629e-2 * t1421 * t11420 - 0.26281718666666666667e-2 * t11423 - 0.4435040025e-2 * t1421 * t11427 - 0.4435040025e-2 * t1421 * t11431 + 0.65704296666666666667e-3 * t1421 * t11435 - 0.22175200125e-2 * t1421 * t11440 - 0.36958666875e-3 * t456 * t11445;
    (t11439, t11444, t11448)
}
