//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1002/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1002<F: Float>(t11997: F, t2778: F, t11937: F, t11781: F, t9999: F, t16182: F, t29033: F, t11483: F, t928: F, t11991: F, t11994: F, t11320: F, t11938: F, t11499: F, t1: F, t102: F, t8448: F) -> (F, F, F, F, F, F, F, F, F) {
    let t33468 = t11997 * t2778;
    let t33470 = t11937 * t2778;
    let t33472 = t11781 * t9999;
    let t33474 = t29033 * t16182;
    let t33476 = t928 * t11483;
    let t33477 = t33476 * t11991;
    let t33479 = t33476 * t11994;
    let t33482 = t928 * t11320 * t11938;
    let t33487 = t928 * t11499 * t11938;
    let t33490 = t8448 * t1 * t102;
    (t33468, t33470, t33472, t33474, t33477, t33479, t33482, t33487, t33490)
}
