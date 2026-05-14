//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1154/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1154<F: Float>(t6904: F, t8248: F, t26763: F, t7030: F, t2389: F, t8229: F, t8331: F, t34239: F, t4391: F, t6964: F, t10525: F, t10526: F, t6689: F, t8411: F, t31590: F, t475: F) -> (F, F, F, F, F, F, F, F) {
    let t34305 = 0.23833659967900284446e0 * t8248 * t6904;
    let t34306 = t26763 * t7030;
    let t34307 = 0.29792074959875355558e-1 * t34306;
    let t34308 = t8229 * t2389;
    let t34309 = 0.59584149919750711116e-1 * t34308;
    let t34310 = t8331 * t2389;
    let t34311 = 0.59584149919750711116e-1 * t34310;
    let t34314 = 0.85801175884441024006e1 * t4391 * t6964 * t34239;
    let t34318 = 0.42900587942220512002e1 * t10525 * t10526 * t34239;
    let t34320 = 0.10725146985555128001e1 * t8411 * t6689;
    let t34321 = t31590 * t475;
    (t34305, t34307, t34309, t34311, t34314, t34318, t34320, t34321)
}
