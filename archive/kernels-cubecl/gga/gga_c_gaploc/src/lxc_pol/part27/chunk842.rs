//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 842/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk842<F: Float>(t1445: F, t8147: F, t1265: F, t2854: F, t2765: F, t524: F, t188: F, t7930: F, t1457: F, t7996: F, t8012: F, t7957: F) -> (F, F, F, F, F, F, F) {
    let t8148 = t1445 * t8147;
    let t8151 = t2854 * t1265;
    let t8152 = t1445 * t8151;
    let t8155 = t524 * t2765;
    let t8158 = t188 * t7930;
    let t8165 = t1457 * t7996;
    let t8168 = t1457 * t8012;
    let t8171 = t1445 * t7957;
    (t8148, t8152, t8155, t8158, t8165, t8168, t8171)
}
