//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 660/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk660<F: Float>(t2240: F, t6489: F, t645: F, t79: F, t72: F, t605: F, t608: F, t38: F, t43: F, t625: F, t44: F, t607: F, t614: F) -> (F, F, F, F, F, F) {
    let t6490 = t2240 * t6489;
    let t6491 = t79 * t645;
    let t6492 = t72 * t6491;
    let t6495 = t605 * t608;
    let t6500 = t38 * t43;
    let t6503 = F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t625;
    let t6504 = -F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t614 * t44 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t6500 * t607 + t6503;
    (t6490, t6492, t6495, t6500, t6503, t6504)
}
