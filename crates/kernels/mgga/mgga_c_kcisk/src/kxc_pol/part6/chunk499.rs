//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 499/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk499<F: Float>(t6387: F, t1556: F, t2306: F, t260: F, t338: F, t67: F, t41: F, t4143: F, t1576: F, t2318: F, t2317: F, t1308: F, t2321: F, t3973: F, t1580: F, t2327: F, t4419: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6388 = t6387 * sigma0;
    let t6426 = t2306 * t1556;
    let t6442 = t260 * t67 * t338;
    let t6443 = t41 * t4143;
    let t6456 = t2318 * t1576;
    let t6458 = t2317 * sigma0;
    let t6459 = t6458 * t1308;
    let t6473 = t3973 * t2321;
    let t6474 = t1580 * t6473;
    let t6497 = t4419 * t2327;
    (t6388, t6426, t6442, t6443, t6456, t6459, t6473, t6474, t6497)
}
