//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 840/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk840<F: Float>(t1163: F, t1165: F, t955: F, t991: F, t315: F, t4197: F, t1162: F, t1172: F, t4198: F, t1077: F, t944: F, t1131: F, t3178: F, t3372: F, t12992: F, t3176: F, t3451: F) -> (F, F, F, F, F, F, F, F) {
    let t13582 = t1163 * t1165 * t991 * t955;
    let t13584 = t315 * t4197;
    let t13585 = t13584 * t1162;
    let t13591 = t4198 * t1172;
    let t13597 = t944 * t1077;
    let t13602 = t944 * t1131;
    let t13627 = t3372 * t3178;
    let t13631 = t3451 * t1165 * t12992 * t3176;
    (t13582, t13584, t13585, t13591, t13597, t13602, t13627, t13631)
}
