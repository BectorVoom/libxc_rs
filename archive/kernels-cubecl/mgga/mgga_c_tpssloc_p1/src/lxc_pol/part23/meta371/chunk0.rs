//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1171/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1171<F: Float>(t3242: F, t415: F, t61: F, t42341: F, t44696: F, t42344: F, t483: F, t1210: F, t44722: F, t478: F, t11147: F, t3439: F) -> (F, F, F, F, F, F) {
    let t44827 = F::cast_from(1.0_f64) / t415 / t3242;
    let t44828 = t61 * t44827;
    let t44833 = t44696 * t42341;
    let t44834 = t483 * t42344;
    let t44836 = t44833 * t1210 * t44834;
    let t44863 = t44833 * t44722 * t478 * t44834;
    let t44938 = t3439 * t11147;
    (t44828, t44833, t44834, t44836, t44863, t44938)
}
