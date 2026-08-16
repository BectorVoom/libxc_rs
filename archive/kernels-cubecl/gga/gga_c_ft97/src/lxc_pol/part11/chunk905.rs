//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 905/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk905<F: Float>(t2: F, t37355: F, t37357: F, t1797: F, t8282: F, t1783: F, t1793: F, t1775: F, t8316: F, t8276: F, t8267: F, t1587: F, t1780: F, t38550: F, t38554: F, t38556: F, t38560: F, t38562: F, t38566: F, t38570: F, t462: F, t463: F, t8183: F, t8261: F, t8275: F) -> (F, F, F) {
    let t38571 = t2 * t37355;
    let t38572 = t38571 * t37357;
    let t38576 = t8282 * t1797;
    let t38578 = t8282 * t1783;
    let t38584 = t8282 * t1793;
    let t38586 = t1775 * t8316;
    let t38588 = t8276 * t37357;
    let t38592 = t1775 * t8267;
    let t38594 = F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t462 * t8275 * t38550 + F::cast_from(112.0_f64) / F::cast_from(81.0_f64) * t38554 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t462 * t1780 * t38556 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t38560 + F::cast_from(8.0_f64) * t462 * t463 * t38562 + F::cast_from(2.0_f64) * t462 * t463 * t38566 - F::cast_from(80.0_f64) / F::cast_from(81.0_f64) * t462 * t38570 * t38572 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t38576 - F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t38578 + F::cast_from(8.0_f64) * t462 * t1587 * t8261 * t8183 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t38584 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t38586 - F::cast_from(8.0_f64) * t462 * t1780 * t38588 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t38592;
    (t38572, t38588, t38594)
}
