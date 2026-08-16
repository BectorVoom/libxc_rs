//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1002/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1002<F: Float>(t11986: F, t475: F, t1445: F, t11982: F, t11987: F, t11977: F, t188: F, t1457: F, t3701: F, t528: F, t1: F, t3689: F) -> (F, F, F, F, F, F, F, F) {
    let t12044 = t11986 * t475;
    let t12045 = t1445 * t12044;
    let t12048 = t1445 * t11982;
    let t12051 = t1445 * t11987;
    let t12054 = t188 * t11977;
    let t12057 = t1457 * t11982;
    let t12060 = t528 * t3701;
    let t12063 = t3689 * t1;
    (t12044, t12045, t12048, t12051, t12054, t12057, t12060, t12063)
}
