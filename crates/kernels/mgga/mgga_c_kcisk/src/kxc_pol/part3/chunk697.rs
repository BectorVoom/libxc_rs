//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 697/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk697<F: Float>(t10534: F, t5290: F, t5289: F, t10381: F, t7315: F, t10431: F, t4816: F, t5320: F, t5323: F, t11671: F, t7316: F, t5322: F, t7429: F, t11646: F, t716: F, t740: F) -> (F, F, F, F, F, F, F) {
    let t11708 = t5290 * t10534;
    let t11709 = t5289 * t11708;
    let t11711 = t5290 * t10381;
    let t11712 = t7315 * t11711;
    let t11714 = t5290 * t10431;
    let t11715 = t5289 * t11714;
    let t11717 = t4816 * t5320;
    let t11718 = t11717 * t5323;
    let t11720 = t7316 * t11671;
    let t11721 = t7315 * t11720;
    let t11723 = t5322 * t10381;
    let t11724 = t7429 * t11723;
    let t11726 = t11646 * t716;
    let t11727 = t11726 * t740;
    (t11709, t11712, t11715, t11718, t11721, t11724, t11727)
}
