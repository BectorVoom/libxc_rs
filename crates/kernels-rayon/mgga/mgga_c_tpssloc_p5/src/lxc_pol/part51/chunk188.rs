//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 188/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk188(t59: f64, t625: f64, t39: f64, t44: f64, t51: f64, t615: f64, t618: f64, t621: f64, t33: f64, t40: f64, t73: f64, t52: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t626 = t59 * t625;
    let t627 = 8.0_f64 / 3.0_f64 * t626;
    let t628 = -8.0_f64 / 3.0_f64 * t615 * t44 + 5.0_f64 / 6.0_f64 * t39 * t618 - 5.0_f64 / 6.0_f64 * t51 * t621 + t627;
    let t629 = t33 * t628;
    let t632 = t40 * t40;
    let t634 = 1.0_f64 / t73 / t632;
    let t636 = t52 * t52;
    (t626, t627, t628, t629, t632, t634, t636)
}
