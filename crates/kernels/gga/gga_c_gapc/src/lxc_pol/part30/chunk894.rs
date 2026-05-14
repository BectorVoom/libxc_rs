//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 894/1135 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk894<F: Float>(t12328: F, t12330: F, t12332: F, t12334: F, t12336: F, t12338: F, t12341: F, t12344: F, t12345: F, t12348: F, t12434: F, t12572: F, t224: F, t3899: F, t987: F, t3707: F, t435: F) -> (F, F, F, F) {
    let t12573 = -t12328 + t12330 + t12332 - t12334 + t12336 - t12338 + t12341 - t12344 + t12345 - t12348 + t12434;
    let t12574 = t12572 + t12573;
    let t12575 = t224 * t12574;
    let t12664 = t987 * t3899;
    let t12744 = t435 * t3707;
    (t12574, t12575, t12664, t12744)
}
