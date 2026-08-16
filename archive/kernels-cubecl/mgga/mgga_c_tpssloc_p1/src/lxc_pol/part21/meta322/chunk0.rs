//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1690/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1690<F: Float>(t3681: F, t67: F, t758: F, t1294: F, t9905: F, t9892: F, t3826: F, t588: F, t3684: F, t9467: F, t118: F, t1284: F) -> (F, F, F, F, F, F, F) {
    let t12099 = t3681 * t67;
    let t12100 = t12099 * t758;
    let t12103 = F::cast_from(0.35089341735807877242e1_f64) * t1294 * t9905;
    let t12105 = F::cast_from(0.51947577317044391277e2_f64) * t1294 * t9892;
    let t12106 = t588 * t3826;
    let t12109 = F::cast_from(0.21687162600603479684e-1_f64) * t3684 * t9467;
    let t12110 = t1284 * t118;
    (t12099, t12100, t12103, t12105, t12106, t12109, t12110)
}
