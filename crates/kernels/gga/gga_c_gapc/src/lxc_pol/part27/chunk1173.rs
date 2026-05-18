//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1173/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1173<F: Float>(t11488: F, t1688: F, t21115: F, t11361: F, t3060: F, t9272: F, t11604: F, t26759: F, t11326: F, t27420: F, t11308: F, t11325: F, t2993: F) -> (F, F, F, F, F, F) {
    let t34492 = t11488 * t1688 * t21115;
    let t34495 = t3060 * t11361 * t9272;
    let t34497 = t11604 * t26759;
    let t34499 = t11326 * t27420;
    let t34501 = t11326 * t11308;
    let t34503 = t2993 * t11325;
    (t34492, t34495, t34497, t34499, t34501, t34503)
}
