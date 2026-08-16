//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2538/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2538<F: Float>(t1682: F, t3357: F, t11310: F, t1694: F, t3401: F, t11420: F, t1098: F, t14956: F, t1119: F, t14845: F, t3308: F, t3312: F, t4737: F) -> (F, F, F, F, F, F, F) {
    let t51382 = t3357 * t1682;
    let t51385 = t11310 * t1694;
    let t51389 = t3401 * t1694;
    let t51392 = t11420 * t1682;
    let t51397 = t14956 * t1098;
    let t51399 = F::cast_from(3.0_f64) * t51397 * t1119;
    let t51401 = F::cast_from(3.0_f64) * t14845 * t3308;
    let t51402 = t4737 * t3312;
    (t51382, t51385, t51389, t51392, t51399, t51401, t51402)
}
