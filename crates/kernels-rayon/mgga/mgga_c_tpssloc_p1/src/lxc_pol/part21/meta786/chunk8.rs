//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2734/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2734(t19731: f64, t562: f64, t12267: f64, t1336: f64, t1352: f64, t1383: f64, t16033: f64, t16036: f64, t16060: f64, t16136: f64, t16429: f64, t19739: f64, t19805: f64, t20014: f64, t3856: f64, t3897: f64, t5234: f64, t5250: f64, t5287: f64, t5334: f64, t5344: f64, t5349: f64, t564: f64, t56914: f64, t57465: f64, t57545: f64, t57618: f64, t6454: f64) -> (f64, f64) {
    let t57704 = t562 * t19731;
    let t57725 = 4.0_f64 * t1336 * t3897 * t56914 - 4.0_f64 * t1352 * t5344 * t57545 - 2.0_f64 * t1352 * t5344 * t57618 - 4.0_f64 * t16036 * t5287 * t5344 - 2.0_f64 * t19739 * t3856 * t5344 + 4.0_f64 * t5250 * t5334 * t57704 - t12267 * t6454 + 2.0_f64 * t1383 * t19805 - 4.0_f64 * t16033 * t20014 - 4.0_f64 * t16060 * t5349 - 2.0_f64 * t16136 * t5234 + 4.0_f64 * t16429 * t5234 + t564 * t57465;
    (t57704, t57725)
}
