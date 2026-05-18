//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1171/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1171<F: Float>(t11526: F, t26778: F, t11522: F, t21655: F, t26369: F, t34419: F, t5541: F, t8677: F, t20768: F, t26561: F, t1030: F, t27354: F, t34186: F) -> (F, F, F, F, F) {
    let t34522 = t11526 * t26778;
    let t34525 = t21655 * t11522 * t26369;
    let t34528 = t5541 * t34419 * t8677;
    let t34530 = t26561 * t20768;
    let t34533 = t1030 * t34186 * t27354;
    (t34522, t34525, t34528, t34530, t34533)
}
