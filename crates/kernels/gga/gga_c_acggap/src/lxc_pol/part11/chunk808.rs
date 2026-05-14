//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 808/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk808<F: Float>(t30468: F, t3393: F, t7361: F, t7433: F, t7353: F, t1181: F, t16548: F, t599: F, t7346: F, t7832: F, t7423: F, t30216: F, t7588: F, t30374: F, t7428: F, t121: F, t413: F) -> (F, F, F, F, F, F, F, F, F) {
    let t30469 = t30468 * t3393;
    let t30497 = t7433 * t7361;
    let t30507 = t7433 * t7353;
    let t30511 = t7346 * t1181 * t599 * t16548;
    let t30522 = t7433 * t7832;
    let t30524 = t7433 * t7423;
    let t30534 = t30216 * t7588;
    let t30536 = t30374 * t7428;
    let t30538 = t121 * t413;
    (t30469, t30497, t30507, t30511, t30522, t30524, t30534, t30536, t30538)
}
