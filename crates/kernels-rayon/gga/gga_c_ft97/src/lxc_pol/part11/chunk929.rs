//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 929/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk929(t376: f64, t8462: f64, t89: f64, t100: f64, t38482: f64, t1841: f64, t463: f64, t1882: f64, t8357: f64, t11987: f64, t11988: f64, t1643: f64, t1901: f64, t1912: f64, t3193: f64, t3194: f64, t38660: f64, t38930: f64, t38947: f64, t38960: f64, t432: f64, t446: f64, t452: f64, t488: f64, t499: f64, t8183: f64, t83: f64, t8355: f64, t8367: f64, t8368: f64, t8506: f64) -> f64 {
    let t39270 = t89 * t376 * t8462;
    let t39272 = t38482 * t100;
    let t39285 = t463 * t1841;
    let t39304 = t1882 * t8357;
    let t39310 = -4.0_f64 / 9.0_f64 * t39270 + 40.0_f64 / 81.0_f64 * t1901 * t39272 * t11988 * t38947 - 8.0_f64 * t446 * t83 * t38660 + 4.0_f64 / 3.0_f64 * t446 * t452 * t488 * t8355 * t432 + 4.0_f64 / 3.0_f64 * t1901 * t39285 * t1912 - 8.0_f64 / 27.0_f64 * t1901 * t3193 * t3194 * t38960 - 20.0_f64 / 27.0_f64 * t1901 * t11987 * t11988 * t38930 - 8.0_f64 / 3.0_f64 * t1901 * t8506 * t8368 - 8.0_f64 / 9.0_f64 * t1901 * t3193 * t8367 * t1643 + 4.0_f64 / 9.0_f64 * t39304 - 4.0_f64 / 3.0_f64 * t446 * t452 * t499 * t8183;
    t39310
}
