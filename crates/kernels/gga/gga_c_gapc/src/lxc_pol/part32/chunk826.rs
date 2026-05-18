//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 826/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk826<F: Float>(t9605: F, t9606: F, t197: F, t7776: F, t1077: F, t129: F, t7624: F, t7626: F, t2622: F, t3336: F, t7595: F, t9435: F) -> (F, F, F, F, F) {
    let t9607 = t9605 * t9606;
    let t9609 = t197 * t7776;
    let t9610 = t1077 * t9609;
    let t9612 = t7624 * t129;
    let t9613 = t197 * t7626;
    let t9614 = t9612 * t9613;
    let t9616 = t3336 * t2622;
    let t9618 = t9435 * t7595;
    (t9607, t9610, t9614, t9616, t9618)
}
