//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 911/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk911<F: Float>(t10019: F, t2617: F, t3005: F, t7810: F, t10628: F, t4820: F, t7513: F, t1029: F, t7803: F, t1052: F, t7822: F, t2972: F, t7324: F, t3459: F, t5552: F, t3073: F, t977: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t11108 = 0.15976219147466979032e-1 * t10019;
    let t11109 = t3005 * t2617;
    let t11110 = t7810 * t11109;
    let t11111 = 0.19171462976960374838e0 * t11110;
    let t11116 = t4820 * t10628;
    let t11118 = 0.79445533226334281487e-1 * t7513 * t11116;
    let t11119 = t1029 * t2617;
    let t11120 = t7803 * t11119;
    let t11121 = 0.19171462976960374838e0 * t11120;
    let t11130 = t7822 * t1052;
    let t11132 = 2.0 * t7324 * t2972;
    let t11134 = 2.0 * t5552 * t3459;
    let t11135 = t3073 * t977;
    (t11108, t11109, t11111, t11116, t11118, t11119, t11121, t11130, t11132, t11134, t11135)
}
