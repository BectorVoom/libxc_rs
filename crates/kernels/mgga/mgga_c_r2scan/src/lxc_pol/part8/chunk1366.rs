//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1366/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1366<F: Float>(t21097: F, t21102: F, t21104: F, t26374: F, t26376: F, t26378: F, t26382: F, t26384: F, t26386: F, t26388: F, t26390: F, t28476: F, t28479: F, t28495: F, t21107: F, t21110: F, t21112: F, t21119: F, t21121: F, t26396: F, t26399: F, t28497: F, t28499: F, t28503: F, t28505: F, t28507: F, t28510: F) -> (F, F) {
    let t33423 = t21097 - 0.1016176784e-1 * t28476 + 0.21687162600603479684e-1 * t28479 + 0.4572795528e-1 * t26374 + 0.2032353568e-1 * t26376 - 0.50603379401408119263e-1 * t26378 - t26382 - 0.762132588e-2 * t26384 - 0.1524265176e-1 * t26386 - t26388 - t21102 + t26390 - t21104 + 0.65061487801810439052e-1 * t28495;
    let t33433 = 0.4051561992e0 * t28497 + 0.8103123984e0 * t28499 + 0.4051561992e0 * t28503 + 0.8103123984e0 * t28505 + 0.8103123984e0 * t28507 + t21107 - t21110 - t21112 + t21119 + 36.0 * t28510 - 0.16008171603946666666e-1 * t21121 - 0.36464057928e1 * t26396 + t26399;
    (t33423, t33433)
}
