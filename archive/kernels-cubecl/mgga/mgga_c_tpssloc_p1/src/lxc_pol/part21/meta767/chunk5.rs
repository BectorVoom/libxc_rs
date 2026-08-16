//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2650/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2650<F: Float>(t2281: F, t5489: F, t5465: F, t19474: F, t626: F, t19483: F, t19477: F, t12808: F, t19473: F, t19482: F, t19529: F, t2331: F, t2332: F, t2358: F, t26129: F, t29903: F, t4043: F, t4067: F, t45435: F, t45676: F, t5464: F, t5488: F, t64: F, t666: F, t9365: F) -> F {
    let t55531 = t2281 * t5489;
    let t55537 = t2281 * t5465;
    let t55546 = t626 * t19474;
    let t55559 = t626 * t19483;
    let t55561 = t626 * t19477;
    let t55566 = -F::cast_from(11.0_f64) / F::cast_from(9.0_f64) * t55531 + F::cast_from(4.0_f64) * t45676 - F::cast_from(3.0_f64) * t29903 * t26129 * t4067 + F::cast_from(22.0_f64) / F::cast_from(9.0_f64) * t55537 + t64 * t2331 * t19529 * t666 / F::cast_from(2.0_f64) + t64 * t19482 * t2358 / F::cast_from(4.0_f64) + F::cast_from(4.0_f64) * t55546 + F::cast_from(3.0_f64) * t64 * t45435 * t5464 * t2332 - F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t64 * t9365 * t5488 * t2332 + t64 * t4043 * t12808 / F::cast_from(2.0_f64) - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t55559 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t55561 - F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t64 * t19473 * t2358;
    t55566
}
