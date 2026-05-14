//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 777/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk777<F: Float>(t7664: F, t779: F, t2414: F, t777: F, t216: F, t2374: F, t798: F, t231: F, t2417: F, t2372: F, t774: F, t2375: F, t228: F, t2418: F, t2409: F, t2416: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7666 = 1.0 * t779 * t7664;
    let t7668 = 1.0 / t2414 / t777;
    let t7669 = t216 * t7668;
    let t7670 = t2374 * t798;
    let t7672 = 1.0 / t2417 / t231;
    let t7673 = t7670 * t7672;
    let t7675 = 0.51725014705706168417e3 * t7669 * t7673;
    let t7676 = t774 * t2372;
    let t7678 = 6.0 * t7676 * t2375;
    let t7680 = 1.0 / t2414 / t228;
    let t7681 = t216 * t7680;
    let t7682 = t7670 * t2418;
    let t7684 = 0.96490945932906628932e2 * t7681 * t7682;
    let t7686 = t2409 * t2418 * t798;
    let t7688 = 0.48245472966453314466e2 * t2416 * t7686;
    (t7666, t7668, t7669, t7670, t7672, t7673, t7675, t7676, t7678, t7680, t7681, t7682, t7684, t7686, t7688)
}
