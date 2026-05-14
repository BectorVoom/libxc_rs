//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 909/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk909<F: Float>(t14283: F, t532: F, t1569: F, t3228: F, t1008: F, t4886: F, t14106: F, t3670: F, t3216: F, t5101: F, t3382: F, t4414: F, t1101: F, t1181: F, t1579: F, t3361: F) -> (F, F, F, F, F, F, F, F) {
    let t16690 = t14283 * t532;
    let t16692 = t3228 * t1569;
    let t16694 = t1008 * t4886;
    let t16701 = t14106 * t532;
    let t16703 = t3670 * t1569;
    let t16705 = t3216 * t5101;
    let t16707 = t3382 * t4414;
    let t16720 = t3361 * t1181 * t1579 * t1101;
    (t16690, t16692, t16694, t16701, t16703, t16705, t16707, t16720)
}
