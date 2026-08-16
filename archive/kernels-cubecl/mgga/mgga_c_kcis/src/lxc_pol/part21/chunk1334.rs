//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1334/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1334<F: Float>(t393: F, t95564: F, t95595: F, t95634: F, t95681: F, t95718: F, t95753: F, t95780: F, t95811: F, t95846: F, t95878: F, t95920: F, t95954: F, t95983: F, t96011: F, t96034: F, t96063: F, t96099: F, t96135: F, t96166: F, t96200: F, t96232: F, t96265: F, t96286: F, t96313: F, t96344: F, t96369: F, t96397: F, t96424: F, t96452: F, t96474: F, t96506: F, t96536: F) -> F {
    let t96542 = (t96313 + t96286 + t96265 + t96232 + t96200 + t96166 + t96135 + t96099 + t96063 + t96034 + t96011 + t95983 + t95954 + t95920 + t95878 + t95846 + t95811 + t95780 + t95753 + t95718 + t95681 + t95634 + t95595 + t95564 + t96536 + t96506 + t96474 + t96452 + t96424 + t96397 + t96369 + t96344) * t393;
    t96542
}
