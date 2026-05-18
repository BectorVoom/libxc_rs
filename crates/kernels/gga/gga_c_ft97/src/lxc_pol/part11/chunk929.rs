//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 929/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk929<F: Float>(t376: F, t8462: F, t89: F, t100: F, t38482: F, t1841: F, t463: F, t1882: F, t8357: F, t11987: F, t11988: F, t1643: F, t1901: F, t1912: F, t3193: F, t3194: F, t38660: F, t38930: F, t38947: F, t38960: F, t432: F, t446: F, t452: F, t488: F, t499: F, t8183: F, t83: F, t8355: F, t8367: F, t8368: F, t8506: F) -> F {
    let t39270 = t89 * t376 * t8462;
    let t39272 = t38482 * t100;
    let t39285 = t463 * t1841;
    let t39304 = t1882 * t8357;
    let t39310 = -F::new(4.0) / F::new(9.0) * t39270 + F::new(40.0) / F::new(81.0) * t1901 * t39272 * t11988 * t38947 - F::new(8.0) * t446 * t83 * t38660 + F::new(4.0) / F::new(3.0) * t446 * t452 * t488 * t8355 * t432 + F::new(4.0) / F::new(3.0) * t1901 * t39285 * t1912 - F::new(8.0) / F::new(27.0) * t1901 * t3193 * t3194 * t38960 - F::new(20.0) / F::new(27.0) * t1901 * t11987 * t11988 * t38930 - F::new(8.0) / F::new(3.0) * t1901 * t8506 * t8368 - F::new(8.0) / F::new(9.0) * t1901 * t3193 * t8367 * t1643 + F::new(4.0) / F::new(9.0) * t39304 - F::new(4.0) / F::new(3.0) * t446 * t452 * t499 * t8183;
    t39310
}
