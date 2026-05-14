//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 817/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk817<F: Float>(t1783: F, t8282: F, t1793: F, t1775: F, t8316: F, t37357: F, t8276: F, t8267: F, t1587: F, t1780: F, t38550: F, t38554: F, t38556: F, t38560: F, t38562: F, t38566: F, t38570: F, t38572: F, t38576: F, t462: F, t463: F, t8183: F, t8261: F, t8275: F) -> (F, F) {
    let t38578 = t8282 * t1783;
    let t38584 = t8282 * t1793;
    let t38586 = t1775 * t8316;
    let t38588 = t8276 * t37357;
    let t38592 = t1775 * t8267;
    let t38594 = 40.0 / 9.0 * t462 * t8275 * t38550 + 112.0 / 81.0 * t38554 - 2.0 / 3.0 * t462 * t1780 * t38556 + 8.0 / 3.0 * t38560 + 8.0 * t462 * t463 * t38562 + 2.0 * t462 * t463 * t38566 - 80.0 / 81.0 * t462 * t38570 * t38572 - 8.0 / 9.0 * t38576 - 16.0 / 27.0 * t38578 + 8.0 * t462 * t1587 * t8261 * t8183 + 16.0 / 9.0 * t38584 - 16.0 / 9.0 * t38586 - 8.0 * t462 * t1780 * t38588 + 4.0 / 9.0 * t38592;
    (t38588, t38594)
}
