//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1417/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1417<F: Float>(t41: F, t7007: F, t725: F, t22336: F, t22340: F, t22335: F, t22344: F, t159: F, t170: F, t22426: F, t22429: F, t22431: F, t22433: F, t22435: F, t22437: F, t22441: F, t22443: F, t22446: F, t22449: F, t22450: F) -> (F, F) {
    let t26801 = t41 * t7007 * t725;
    let t26803 = 31680.0 * t22336;
    let t26804 = 52416.0 * t22340;
    let t26805 = t22335 - t26803 + t26804 - t22344;
    let t26812 = 0.51947577317044391277e2 * t22426 + 0.8103123984e0 * t22429 + 0.15584273195113317383e3 * t22431 + 0.92286169723947659919e4 * t22433 + 0.10526802520742363173e2 * t22435 + 0.1870112783413598086e4 * t22437 - 3.0 * t26801 + 0.285764e-1 * t159 * t26805 * t170 + 360.0 * t22441 + 0.19518446340543131715e0 * t22443 + t22446 - t22449 - 0.18676200204604444443e-1 * t22450;
    (t26805, t26812)
}
