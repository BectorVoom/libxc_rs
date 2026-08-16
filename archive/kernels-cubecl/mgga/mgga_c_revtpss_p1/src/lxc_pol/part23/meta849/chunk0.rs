//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2732/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2732<F: Float>(t1285: F, t70994: F, t17384: F, t17605: F, t17448: F, t17451: F, t1121: F, t6587: F, t13148: F, t70916: F, t13142: F, t12772: F, t21218: F, t3625: F) -> (F, F, F, F, F, F, F) {
    let t70995 = t1285 * t70994;
    let t71009 = t17605 * t17384;
    let t71020 = t17448 * t17451;
    let t71029 = t6587 * t1121;
    let t71036 = t13148 * t70916;
    let t71039 = t13142 * t70916;
    let t71047 = t3625 * t12772 * t21218;
    (t70995, t71009, t71020, t71029, t71036, t71039, t71047)
}
