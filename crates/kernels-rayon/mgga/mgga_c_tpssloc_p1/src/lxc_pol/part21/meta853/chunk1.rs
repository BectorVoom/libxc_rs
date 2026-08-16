//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3083/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3083(t1113: f64, t136: f64, t63406: f64, t50826: f64, t50828: f64, t50834: f64, t63291: f64, t63296: f64, t63300: f64, t63304: f64, t63306: f64, t63308: f64, t63313: f64, t63317: f64, t63323: f64) -> (f64, f64) {
    let t63939 = t136 * t1113 * t63406;
    let t63953 = -4.0_f64 / 9.0_f64 * t63291 + 4.0_f64 / 3.0_f64 * t63296 + 2.0_f64 / 3.0_f64 * t63300 + 2.0_f64 * t63304 + 4.0_f64 / 27.0_f64 * t63306 - 20.0_f64 / 81.0_f64 * t63308 - 4.0_f64 / 9.0_f64 * t63313 - 2.0_f64 / 9.0_f64 * t63317 + 16.0_f64 / 27.0_f64 * t50826 - 2.0_f64 / 9.0_f64 * t50828 - 56.0_f64 / 81.0_f64 * t50834 + 40.0_f64 / 27.0_f64 * t63323;
    (t63939, t63953)
}
