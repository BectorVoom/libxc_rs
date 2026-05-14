//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 769/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk769<F: Float>(t129: F, t7624: F, t197: F, t7626: F, t2622: F, t3336: F, t7595: F, t9435: F, t2300: F, t2636: F, t3396: F, t2979: F, t2255: F, t2982: F, t2619: F, t9128: F) -> (F, F, F, F, F, F, F) {
    let t9612 = t7624 * t129;
    let t9613 = t197 * t7626;
    let t9614 = t9612 * t9613;
    let t9616 = t3336 * t2622;
    let t9618 = t9435 * t7595;
    let t9620 = t2636 * t2300;
    let t9621 = t3396 * t9620;
    let t9623 = t7624 * t2979;
    let t9624 = t2982 * t2255;
    let t9625 = t9623 * t9624;
    let t9627 = t2619 * t9128;
    (t9614, t9616, t9618, t9621, t9624, t9625, t9627)
}
