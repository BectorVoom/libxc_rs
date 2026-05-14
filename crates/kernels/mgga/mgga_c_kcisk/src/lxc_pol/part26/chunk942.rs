//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 942/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk942<F: Float>(t12941: F, t25465: F, t26: F, t25469: F, t3661: F, t5744: F, t1186: F, t25441: F, t25450: F, t25432: F, t25437: F, t12969: F, t7757: F, t1175: F, t5684: F, t5730: F) -> (F, F, F, F, F, F, F, F) {
    let t25715 = t12941 * t25465;
    let t25716 = t26 * t25715;
    let t25718 = t3661 * t25469;
    let t25719 = t5744 * t25718;
    let t25721 = t1186 * t25441;
    let t25722 = t26 * t25721;
    let t25724 = t1186 * t25450;
    let t25725 = t5744 * t25724;
    let t25727 = t1186 * t25432;
    let t25728 = t26 * t25727;
    let t25730 = t3661 * t25437;
    let t25731 = t26 * t25730;
    let t25742 = t12969 * t7757;
    let t25743 = t25742 * t1175;
    let t25745 = t5730 * t5684;
    (t25716, t25719, t25722, t25725, t25728, t25731, t25743, t25745)
}
