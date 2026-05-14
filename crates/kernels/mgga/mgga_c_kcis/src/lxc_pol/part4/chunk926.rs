//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 926/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk926<F: Float>(t1401: F, t4039: F, t4142: F, t4178: F, t25: F, t4008: F, t493: F, t499: F, t737: F, t110: F, t1381: F, t109: F, t1369: F, t1372: F, t1368: F, t3970: F, t3994: F) -> (F, F, F, F, F, F, F, F) {
    let t12091 = t1401 * t4039;
    let t12119 = t4142 * t4178;
    let t12124 = t25 * t4008;
    let t12125 = t493 * t12124;
    let t12127 = t737 * t499;
    let t12129 = 5.0 / 1296.0 * t493 * t12127;
    let t12130 = t110 * t1381;
    let t12131 = t493 * t12130;
    let t12133 = t109 * t1369;
    let t12134 = t12133 * t1372;
    let t12135 = t1368 * t12134;
    let t12137 = t3970 * t3994;
    (t12091, t12119, t12125, t12129, t12131, t12133, t12135, t12137)
}
