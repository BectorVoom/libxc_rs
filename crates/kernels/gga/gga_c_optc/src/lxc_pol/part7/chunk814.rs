//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 814/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk814<F: Float>(t7647: F, t7662: F, t799: F, t779: F, t2414: F, t777: F, t216: F, t2374: F, t798: F, t231: F, t2417: F, t2372: F, t774: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7663 = t7647 + t7662;
    let t7664 = t7663 * t799;
    let t7666 = F::new(1.0) * t779 * t7664;
    let t7668 = F::new(1.0) / t2414 / t777;
    let t7669 = t216 * t7668;
    let t7670 = t2374 * t798;
    let t7672 = F::new(1.0) / t2417 / t231;
    let t7673 = t7670 * t7672;
    let t7675 = F::new(0.51725014705706168417e3) * t7669 * t7673;
    let t7676 = t774 * t2372;
    (t7663, t7664, t7666, t7668, t7669, t7670, t7672, t7673, t7675, t7676)
}
