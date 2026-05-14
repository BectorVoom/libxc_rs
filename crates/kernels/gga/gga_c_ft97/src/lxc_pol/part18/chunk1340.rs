//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1340/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1340<F: Float>(t27078: F, t95053: F, t2185: F, t23652: F, t23657: F, t27152: F, t1369: F, t27047: F, t376: F, t1580: F, t1969: F, t27034: F, t446: F, t1557: F, t6615: F, t1559: F, t9049: F) -> (F, F, F, F, F, F, F) {
    let t105809 = t95053 * t27078;
    let t105810 = t105809 / 18.0;
    let t105813 = t23657 * t2185 * t23652 * t27152;
    let t105815 = t1369 * t376 * t27047;
    let t105816 = t105815 / 3.0;
    let t105819 = t446 * t1969 * t27034 * t1580;
    let t105821 = t6615 * t1557;
    let t105824 = t446 * t9049 * t105821 * t1559;
    (t105809, t105810, t105813, t105815, t105816, t105819, t105824)
}
