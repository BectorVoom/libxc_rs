//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1982/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1982<F: Float>(t16205: F, t550: F, t1343: F, t820: F, t12365: F, t1827: F, t12300: F, t1799: F, t3734: F, t12351: F, t12418: F) -> (F, F, F, F, F, F) {
    let t16206 = t16205 * t550;
    let t16208 = t1343 * t820 * t16206;
    let t16211 = t12365 * t1827;
    let t16214 = F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t12300 * t1827;
    let t16215 = t1799 * t3734;
    let t16217 = t12351 * t820 * t16215;
    let t16224 = t12418 * t820;
    (t16206, t16208, t16211, t16214, t16217, t16224)
}
