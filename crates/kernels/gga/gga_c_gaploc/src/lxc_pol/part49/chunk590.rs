//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 590/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk590<F: Float>(t1628: F, t3402: F, t10215: F, t600: F, t568: F, t3414: F, t10216: F, t531: F, t569: F, t3371: F, t524: F, t189: F, t188: F, t1589: F, t3358: F, t2482: F, t2890: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10564 = t1628 * t3402;
    let t10569 = t600 * t10215;
    let t10570 = t568 * t10569;
    let t10573 = t1628 * t3414;
    let t10578 = t531 * t10216;
    let t10583 = t569 * t10215;
    let t10584 = t568 * t10583;
    let t10587 = t524 * t3371;
    let t10590 = t189 * t10215;
    let t10591 = t188 * t10590;
    let t10594 = t1589 * t3358;
    let t10597 = t2890 * t2482;
    (t10564, t10570, t10573, t10578, t10584, t10587, t10590, t10591, t10594, t10597)
}
