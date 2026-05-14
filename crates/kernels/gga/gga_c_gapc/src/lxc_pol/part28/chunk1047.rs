//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1047/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1047<F: Float>(t1086: F, t11790: F, t23104: F, t11449: F, t11805: F, t190: F, t761: F, t286: F, t3074: F, t33491: F, t7735: F, t11320: F, t11795: F, t2520: F, t34113: F, t7503: F) -> (F, F, F, F, F) {
    let t34241 = t11790 * t1086 * t23104;
    let t34245 = t761 * t190 * t11449 * t11805;
    let t34247 = t3074 * t286;
    let t34249 = t33491 * t34247 * t7735;
    let t34252 = t2520 * t11320 * t11795;
    let t34255 = t34113 * t34247 * t7503;
    (t34241, t34245, t34249, t34252, t34255)
}
