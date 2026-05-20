//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3126/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3126<F: Float>(t12772: F, t17634: F, t3625: F, t17395: F, t3746: F, t1248: F, t44586: F, t17689: F, t44425: F, t17435: F, t3667: F, t1235: F, t127: F, t17278: F, t371: F) -> (F, F, F, F, F, F) {
    let t57569 = t3625 * t12772 * t17634;
    let t57571 = t3746 * t17395;
    let t57578 = t44586 * t1248;
    let t57584 = t3625 * t44425 * t17689;
    let t57586 = t3667 * t17435;
    let t57590 = t1235 * t371 * t127 * t17278;
    (t57569, t57571, t57578, t57584, t57586, t57590)
}
