//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1215/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1215<F: Float>(t1030: F, t26034: F, t34077: F, t20501: F, t33411: F, t19511: F, t33415: F, t11388: F, t3065: F, t11479: F, t1912: F, t5285: F) -> (F, F, F, F, F) {
    let t35149 = t1030 * t34077 * t26034;
    let t35152 = t1030 * t33411 * t20501;
    let t35155 = t1030 * t33415 * t19511;
    let t35157 = t11388 * t3065;
    let t35160 = t5285 * t11479 * t1912;
    (t35149, t35152, t35155, t35157, t35160)
}
