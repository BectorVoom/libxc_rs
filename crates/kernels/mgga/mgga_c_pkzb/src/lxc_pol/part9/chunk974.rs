//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 974/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk974<F: Float>(t1227: F, t2411: F, t300: F, t1235: F, t297: F, t46: F, t3232: F, t1697: F, t95: F, t1143: F, t799: F, t1054: F, t633: F, t440: F, t973: F, t1255: F, t951: F) -> (F, F, F, F, F, F, F) {
    let t10212 = t2411 * t1227;
    let t10213 = t300 * t10212;
    let t10257 = t1235 * t297 * t46;
    let t10258 = t3232 * t10257;
    let t11817 = t1697 * t95;
    let t12315 = t1143 * t799;
    let t12508 = t1054 * t633;
    let t12584 = t973 * t440;
    let t12845 = t1255 * t951;
    (t10213, t10258, t11817, t12315, t12508, t12584, t12845)
}
