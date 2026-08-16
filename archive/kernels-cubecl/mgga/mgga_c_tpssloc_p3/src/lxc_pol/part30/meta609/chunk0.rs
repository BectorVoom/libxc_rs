//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2002/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2002<F: Float>(t23012: F, t6573: F, t1883: F, t82045: F, t6568: F, t23205: F, t82038: F, t1914: F, t40772: F, t1054: F, t2775: F, t23326: F, t6712: F) -> (F, F, F, F, F, F, F) {
    let t82211 = t23012 * t6573;
    let t82218 = t82045 * t1883;
    let t82219 = F::cast_from(0.27720185200590482541e0_f64) * t82218;
    let t82259 = t23012 * t6568;
    let t82294 = t82038 * t23205;
    let t82312 = t1914 * t40772;
    let t82342 = t1054 * t2775;
    let t82402 = t6712 * t23326;
    (t82211, t82219, t82259, t82294, t82312, t82342, t82402)
}
