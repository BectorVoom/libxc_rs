//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 554/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk554<F: Float>(t45: F, t56: F, t41: F, t42: F, t7936: F, t78: F, t388: F, t391: F, t625: F, t68: F, t72: F, t2247: F, t47: F, t14: F, t1675: F, t172: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8063 = 1.0 / t45 / t56;
    let t8068 = -0.205601884870781893e1 * t41 * t42 * t8063 - 0.13764695059989835716e0 * t7936;
    let t8069 = t78 * t8068;
    let t8070 = t388 * t8069;
    let t8074 = t68 * t391 * t625 * t72;
    let t8076 = t47 * t2247;
    let t8078 = t68 * t8076 * t72;
    let t8079 = 0.70937342644032921812e-2 * t8078;
    let t8082 = t68 * t8063 * t14 * t72;
    let t8086 = t68 * t1675 * t172 * t72;
    (t8063, t8068, t8069, t8070, t8074, t8078, t8079, t8082, t8086)
}
