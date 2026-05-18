//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 783/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk783<F: Float>(t3338: F, t4130: F, t2482: F, t9272: F, t12960: F, t1537: F, t34890: F, t6583: F, t9537: F, t10473: F, t9263: F, t10469: F, t9267: F) -> (F, F, F, F, F) {
    let t41590 = t4130 * t3338;
    let t41592 = t9272 * t41590 * t2482;
    let t41594 = t1537 * t12960;
    let t41606 = t6583 * t34890 * t9537;
    let t41609 = t9263 * t10473 * t2482;
    let t41612 = t9267 * t10469 * t2482;
    (t41592, t41594, t41606, t41609, t41612)
}
