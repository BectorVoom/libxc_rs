//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1884/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1884<F: Float>(t1827: F, t91285: F, t22756: F, t6417: F, t19868: F, t6945: F, t19815: F, t6944: F, t1354: F, t91278: F, t26233: F, t5289: F) -> (F, F, F, F, F, F) {
    let t97240 = t91285 * t1827;
    let t97242 = t22756 * t6417;
    let t97244 = t6945 * t19868;
    let t97246 = t19815 * t6944;
    let t97247 = t97246 * t1354;
    let t97249 = t91278 * t1827;
    let t97251 = t26233 * t5289;
    (t97240, t97242, t97244, t97247, t97249, t97251)
}
