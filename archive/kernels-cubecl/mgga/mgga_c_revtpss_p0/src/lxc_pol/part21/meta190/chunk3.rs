//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1154/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1154<F: Float>(t1580: F, t779: F, t689: F, t1579: F, t72: F, t686: F, t2465: F, t886: F, t2770: F, t1558: F, t251: F) -> (F, F, F, F, F, F, F) {
    let t4477 = t779 * t1580;
    let t4478 = t689 * t4477;
    let t4480 = t1579 * t72;
    let t4481 = t4480 * t686;
    let t4482 = t2465 * t4481;
    let t4486 = t1579 * t886;
    let t4487 = t2770 * t4486;
    let t4494 = t251 * t1558;
    (t4477, t4478, t4480, t4481, t4482, t4487, t4494)
}
