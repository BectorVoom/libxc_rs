//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 981/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk981<F: Float>(t1775: F, t9225: F, t9183: F, t355: F, t7368: F, t2: F, t9199: F, t9193: F, t525: F, t7760: F, t9218: F, t2102: F, t3499: F, t3506: F, t37315: F, t37320: F, t39719: F, t39726: F, t39735: F, t39751: F, t39755: F, t39759: F, t39765: F, t462: F, t9192: F, t9217: F) -> (F, F, F) {
    let t40405 = t1775 * t9225;
    let t40413 = t1775 * t9183;
    let t40424 = t355 * t7368;
    let t40425 = t40424 * t2;
    let t40432 = t1775 * t9199;
    let t40434 = t1775 * t9193;
    let t40436 = t7760 * t525;
    let t40437 = t40436 * t2;
    let t40444 = t1775 * t9218;
    let t40446 = F::new(40.0) / F::new(81.0) * t40405 + F::new(4.0) / F::new(3.0) * t462 * t2102 * t39735 + F::new(4.0) / F::new(3.0) * t462 * t9192 * t39759 - F::new(4.0) / F::new(3.0) * t40413 - F::new(4.0) * t462 * t2102 * t39755 + F::new(8.0) * t462 * t2102 * t39719 - F::new(12.0) * t462 * t3506 * t37315 + F::new(8.0) * t462 * t40425 * t39751 + F::new(8.0) * t462 * t9217 * t39765 + F::new(8.0) / F::new(9.0) * t40432 - F::new(8.0) / F::new(9.0) * t40434 + F::new(40.0) / F::new(27.0) * t462 * t40437 * t39726 + F::new(8.0) * t462 * t3499 * t37320 + F::new(8.0) / F::new(3.0) * t40444;
    (t40424, t40436, t40446)
}
