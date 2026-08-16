//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2949/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2949<F: Float>(t13822: F, t17752: F, t973: F, t17753: F, t17758: F, t17778: F, t2960: F, t2986: F, t3008: F, t343: F, t4510: F, t4518: F, t4546: F, t5842: F, t59755: F, t59763: F, t61391: F, t61394: F, t61397: F, t61405: F, t61408: F) -> F {
    let t61422 = t973 * t13822 * t17752;
    let t61424 = -F::cast_from(0.74074074074074074073e-3_f64) * t61391 - F::cast_from(0.14814814814814814814e-2_f64) * t61394 + F::cast_from(0.37037037037037037036e-3_f64) * t61397 + F::cast_from(0.16666666666666666666e-2_f64) * t2986 * t4518 * t59763 + F::cast_from(0.13333333333333333332e-1_f64) * t2986 * t4510 * t59755 - F::cast_from(0.49382716049382716048e-3_f64) * t61405 + F::cast_from(0.12345679012345679012e-3_f64) * t61408 + F::cast_from(0.88888888888888888887e-2_f64) * t2960 * t17753 + F::cast_from(0.44444444444444444444e-2_f64) * t2960 * t17758 - F::cast_from(0.83333333333333333332e-3_f64) * t973 * t4546 * t5842 * t3008 * t343 + F::cast_from(0.44444444444444444444e-2_f64) * t2960 * t17778 - F::cast_from(0.11111111111111111111e-2_f64) * t61422;
    t61424
}
