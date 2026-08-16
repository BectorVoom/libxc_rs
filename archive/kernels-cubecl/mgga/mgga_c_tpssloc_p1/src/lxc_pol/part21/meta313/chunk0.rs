//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1675/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1675<F: Float>(t248: F, t3509: F, t3570: F, t3506: F, t135: F, t3561: F, t1174: F, t3247: F, t415: F, t61: F, t121: F, t3584: F) -> (F, F, F, F, F, F, F) {
    let t11745 = t248 * t3570 * t3509;
    let t11746 = t3506 * t11745;
    let t11754 = t135 * t3561;
    let t11755 = t1174 * t11754;
    let t11778 = F::cast_from(1.0_f64) / t415 / t3247;
    let t11779 = t61 * t11778;
    let t11784 = t121 * t3584;
    (t11745, t11746, t11754, t11755, t11778, t11779, t11784)
}
