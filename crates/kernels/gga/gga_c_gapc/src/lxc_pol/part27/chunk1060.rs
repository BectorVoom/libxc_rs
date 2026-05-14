//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1060/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1060<F: Float>(t1030: F, t1971: F, t9267: F, t9272: F, t19522: F, t33623: F, t5541: F, t11303: F, t19588: F, t1743: F, t33373: F, t5967: F, t20200: F, t27307: F, t27309: F, t33399: F, t8362: F) -> (F, F, F, F, F, F, F) {
    let t35105 = t1030 * t1971 * t9267 * t9272;
    let t35108 = t5541 * t33623 * t19522;
    let t35110 = t11303 * t19588;
    let t35112 = t1743 * t33373;
    let t35113 = t35112 * t5967;
    let t35115 = t11303 * t20200;
    let t35119 = t27307 * t33399 * t8362 * t27309;
    (t35105, t35108, t35110, t35112, t35113, t35115, t35119)
}
