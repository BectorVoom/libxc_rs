//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 762/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk762<F: Float>(t11738: F, t5289: F, t11671: F, t7430: F, t7429: F, t11658: F, t740: F, t1950: F, t1945: F, t5332: F, t10522: F, t642: F) -> (F, F, F, F, F) {
    let t11739 = t5289 * t11738;
    let t11741 = t7430 * t11671;
    let t11742 = t7429 * t11741;
    let t11744 = t11658 * t740;
    let t11745 = t11744 * t1950;
    let t11747 = t1945 * t5332;
    let t11749 = t642 * t10522;
    (t11739, t11742, t11745, t11747, t11749)
}
