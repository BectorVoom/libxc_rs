//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 964/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk964<F: Float>(t11662: F, t11663: F, t3235: F, t3729: F, t125: F, t818: F, t329: F, t2536: F, t1062: F, t268: F, t3643: F, t128: F, t6939: F) -> (F, F, F, F, F, F, F) {
    let t11664 = t11662 * t11663;
    let t11666 = t3235 * t3729;
    let t11668 = t125 * t818;
    let t11669 = t11668 * t329;
    let t11670 = t11669 * t2536;
    let t11671 = t1062 * t11670;
    let t11673 = t3643 * t268;
    let t11674 = t6939 * t128;
    (t11664, t11666, t11669, t11670, t11671, t11673, t11674)
}
