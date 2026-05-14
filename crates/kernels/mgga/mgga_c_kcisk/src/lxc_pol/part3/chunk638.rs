//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 638/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk638<F: Float>(t10634: F, t10680: F, t1676: F, t1685: F, t10542: F, t10543: F, t10549: F, t10554: F, t10559: F, t10563: F, t10566: F, t10602: F, t10604: F, t1674: F, t1686: F, t4757: F, t4783: F) -> (F, F, F) {
    let t10681 = t10634 + t10680;
    let t10683 = t1676 * t10681 * t1685;
    let t10686 = t10542 - 0.17544670192365612213e1 * t10543 * t1686 - 0.17544670192365612213e1 * t4757 * t4783 - 0.51947267698127589897e2 * t1674 * t10549 - 0.35089340384731224426e1 * t1674 * t10554 - t10559 + t10563 - t10566 - t10602 + 0.35089340384731224426e1 * t1674 * t10604 - 0.58482233974552040708e0 * t1674 * t10683;
    (t10681, t10683, t10686)
}
