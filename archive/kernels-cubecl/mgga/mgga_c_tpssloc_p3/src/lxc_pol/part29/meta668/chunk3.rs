//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2233/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2233<F: Float>(t16217: F, t6952: F, t1827: F, t80910: F, t22756: F, t5289: F, t16208: F, t6945: F, t16060: F, t6951: F, t1369: F, t1878: F, t80730: F) -> (F, F, F, F, F, F) {
    let t91183 = t6952 * t16217;
    let t91185 = t80910 * t1827;
    let t91187 = t22756 * t5289;
    let t91189 = t6945 * t16208;
    let t91191 = t16060 * t6951;
    let t91192 = t91191 * t1369;
    let t91194 = t1878 * t80730;
    (t91183, t91185, t91187, t91189, t91192, t91194)
}
