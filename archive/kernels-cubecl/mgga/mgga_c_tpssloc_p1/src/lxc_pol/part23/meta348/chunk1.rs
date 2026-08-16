//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1142/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1142<F: Float>(t40394: F, t40399: F, t535: F, t1314: F, t9580: F, t2566: F, t3732: F, t12214: F, t792: F, t2229: F, t59: F, t60: F) -> (F, F, F, F, F) {
    let t40401 = F::cast_from(0.69444444444444444445e-4_f64) * t40394 * t535 * t40399;
    let t40406 = t9580 * t1314;
    let t40409 = t2566 * t3732;
    let t40412 = t792 * t12214;
    let t40419 = t59 / t60 / t2229;
    (t40401, t40406, t40409, t40412, t40419)
}
