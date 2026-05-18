//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 760/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk760<F: Float>(t11714: F, t5289: F, t4816: F, t5320: F, t5323: F, t11671: F, t7316: F, t7315: F, t10381: F, t5322: F, t7429: F, t11646: F, t716: F) -> (F, F, F, F, F) {
    let t11715 = t5289 * t11714;
    let t11717 = t4816 * t5320;
    let t11718 = t11717 * t5323;
    let t11720 = t7316 * t11671;
    let t11721 = t7315 * t11720;
    let t11723 = t5322 * t10381;
    let t11724 = t7429 * t11723;
    let t11726 = t11646 * t716;
    (t11715, t11718, t11721, t11724, t11726)
}
