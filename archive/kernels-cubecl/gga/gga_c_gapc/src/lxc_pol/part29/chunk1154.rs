//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1154/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1154<F: Float>(t286: F, t3074: F, t33491: F, t7735: F, t11320: F, t11795: F, t2520: F, t34113: F, t7503: F, t15679: F, t33202: F, t3787: F) -> (F, F, F, F) {
    let t34247 = t3074 * t286;
    let t34249 = t33491 * t34247 * t7735;
    let t34252 = t2520 * t11320 * t11795;
    let t34255 = t34113 * t34247 * t7503;
    let t34258 = t33202 * t3787 * t15679;
    (t34249, t34252, t34255, t34258)
}
