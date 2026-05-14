//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1059/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1059<F: Float>(t3140: F, t34040: F, t27935: F, t27940: F, t11598: F, t8765: F, t11387: F, t19916: F, t5553: F, t1030: F, t26034: F, t34077: F, t20501: F, t33411: F, t19511: F, t33415: F) -> (F, F, F, F, F, F, F) {
    let t35139 = t34040 * t3140;
    let t35141 = t27935 * t35139 * t27940;
    let t35143 = t11598 * t8765;
    let t35146 = t5553 * t11387 * t19916;
    let t35149 = t1030 * t34077 * t26034;
    let t35152 = t1030 * t33411 * t20501;
    let t35155 = t1030 * t33415 * t19511;
    (t35139, t35141, t35143, t35146, t35149, t35152, t35155)
}
