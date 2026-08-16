//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 981/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk981<F: Float>(t3726: F, t5206: F, t12199: F, t5202: F, t118: F, t5187: F, t794: F, t3739: F, t12225: F, t16095: F, t2586: F, t1338: F, t5318: F) -> (F, F, F, F, F) {
    let t16106 = t3726 * t5206;
    let t16108 = t12199 * t5202;
    let t16111 = t118 * t794 * t5187;
    let t16113 = F::cast_from(0.16666666666666666666e-2_f64) * t3739 * t16111;
    let t16118 = t12225 * t16095;
    let t16119 = t2586 * t16118;
    let t16132 = t1338 * t5318;
    (t16106, t16108, t16113, t16119, t16132)
}
