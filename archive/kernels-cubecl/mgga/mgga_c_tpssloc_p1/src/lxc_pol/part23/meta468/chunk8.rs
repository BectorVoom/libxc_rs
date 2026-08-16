//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1382/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1382<F: Float>(t41655: F, t47787: F, t59657: F, t68442: F, t76574: F, t76578: F, t76583: F, t76587: F, t76591: F, t76595: F, t76599: F, t59688: F, t59694: F, t68444: F, t68446: F, t68448: F, t68494: F, t68498: F, t76610: F, t76614: F, t76618: F, t76622: F, t76626: F) -> (F, F) {
    let t77454 = F::cast_from(0.73871604938271604937e-1_f64) * t47787 - F::cast_from(0.52765432098765432099e-1_f64) * t76574 - F::cast_from(0.17808333333333333333e-1_f64) * t76578 - F::cast_from(0.31659259259259259258e-1_f64) * t59657 + F::cast_from(0.23744444444444444444e0_f64) * t76583 - F::cast_from(0.11872222222222222222e0_f64) * t76587 - F::cast_from(0.42739999999999999999e0_f64) * t76591 + F::cast_from(0.42739999999999999999e0_f64) * t76595 - F::cast_from(0.35616666666666666666e-1_f64) * t76599 + t41655 + F::cast_from(0.14246666666666666667e0_f64) * t68442;
    let t77467 = F::cast_from(0.23744444444444444444e-1_f64) * t68444 + F::cast_from(0.26382716049382716049e-1_f64) * t68446 - F::cast_from(0.94977777777777777776e-1_f64) * t68448 + F::cast_from(0.47488888888888888888e-1_f64) * t68494 - F::cast_from(0.14246666666666666667e0_f64) * t68498 - F::cast_from(0.47488888888888888888e-1_f64) * t76610 + F::cast_from(0.4274e0_f64) * t76614 - F::cast_from(0.6411e0_f64) * t76618 + F::cast_from(0.10685e0_f64) * t76622 + F::cast_from(0.14246666666666666667e0_f64) * t76626 + F::cast_from(0.94977777777777777776e-1_f64) * t59688 - F::cast_from(0.47488888888888888888e-1_f64) * t59694;
    (t77454, t77467)
}
