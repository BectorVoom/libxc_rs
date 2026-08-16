//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 834/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk834<F: Float>(t11135: F, t11203: F, t11153: F, t461: F, t1176: F, t698: F, t135: F, t3439: F, t3247: F, t405: F) -> (F, F, F, F, F, F) {
    let t11459 = F::cast_from(0.55403703703703703703e-1_f64) * t11135;
    let t11487 = F::cast_from(20.0_f64) / F::cast_from(27.0_f64) * t11203;
    let t11516 = t461 * t11153;
    let t11529 = t698 * t1176;
    let t11539 = t135 * t3439;
    let t11545 = F::cast_from(1.0_f64) / t405 / t3247;
    (t11459, t11487, t11516, t11529, t11539, t11545)
}
