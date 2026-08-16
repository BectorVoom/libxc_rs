//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1421/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1421<F: Float>(t22522: F, t9231: F, t2240: F, t22511: F, t33: F, t1865: F, t22513: F, t22519: F, t22523: F, t22527: F, t22544: F, t22546: F, t22549: F, t22554: F, t6492: F, t6495: F, t6506: F, t6510: F, t83734: F, t83738: F, t83741: F, t83745: F, t83748: F) -> F {
    let t83750 = t9231 * t22522;
    let t83760 = t2240 * t33 * t22511;
    let t83766 = -F::cast_from(15.0_f64) * t22544 * t83734 - F::cast_from(5.0_f64) * t22549 * t83738 - F::cast_from(15.0_f64) * t83741 * t22546 - F::cast_from(15.0_f64) * t22544 * t83745 + t83748 * t1865 + F::cast_from(5.0_f64) * t83750 * t6492 + F::cast_from(2.0_f64) * t22519 * t6506 + F::cast_from(5.0_f64) * t22554 * t22527 + F::cast_from(2.0_f64) * t22519 * t6510 + F::cast_from(5.0_f64) / F::cast_from(2.0_f64) * t83760 * t6492 + t6495 * t22513 + F::cast_from(5.0_f64) * t22523 * t22527;
    t83766
}
