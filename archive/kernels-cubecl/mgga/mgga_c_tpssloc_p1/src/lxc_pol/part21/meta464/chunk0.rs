//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2035/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2035<F: Float>(t221: F, t3734: F, t5196: F, t3726: F, t5206: F, t12199: F, t5202: F, t118: F, t5187: F, t794: F, t3739: F, t16018: F, t210: F, t214: F) -> (F, F, F, F, F, F) {
    let t16103 = t221 * t5196 * t3734;
    let t16106 = t3726 * t5206;
    let t16108 = t12199 * t5202;
    let t16111 = t118 * t794 * t5187;
    let t16113 = F::cast_from(0.16666666666666666666e-2_f64) * t3739 * t16111;
    let t16115 = t210 * t214 * t16018;
    (t16103, t16106, t16108, t16111, t16113, t16115)
}
