//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 181/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk181<F: Float>(t59: F, t625: F, t39: F, t44: F, t51: F, t615: F, t618: F, t621: F, t33: F, t40: F, t73: F, t52: F) -> (F, F, F, F, F, F) {
    let t626 = t59 * t625;
    let t627 = F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t626;
    let t628 = -F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t615 * t44 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t39 * t618 - F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t51 * t621 + t627;
    let t629 = t33 * t628;
    let t632 = t40 * t40;
    let t634 = F::cast_from(1.0_f64) / t73 / t632;
    let t636 = t52 * t52;
    (t626, t628, t629, t632, t634, t636)
}
