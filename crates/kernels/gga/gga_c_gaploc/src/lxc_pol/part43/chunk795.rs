//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 795/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk795<F: Float>(t43715: F, t24505: F, t2684: F, t9438: F, t3271: F, t8634: F, t10050: F, t3040: F, t1457: F, t2103: F, t43213: F, t43217: F, t10867: F, t9989: F, t10086: F, t10811: F) -> (F, F, F, F, F, F, F, F) {
    let t43716 = 0.11916829983950142223e0 * t43715;
    let t43718 = t2684 * t9438 * t24505;
    let t43719 = 0.7988109573733489516e-1 * t43718;
    let t43721 = 0.35750489951850426669e0 * t3271 * t8634;
    let t43723 = 0.35750489951850426669e0 * t10050 * t3040;
    let t43726 = 0.71500979903700853338e0 * t2103 * t1457 * t43213;
    let t43729 = 0.71500979903700853338e0 * t2103 * t1457 * t43217;
    let t43735 = 0.25025342966295298669e1 * t10867 * t1457 * t9989;
    let t43737 = 0.42900587942220512003e1 * t10811 * t10086;
    (t43716, t43719, t43721, t43723, t43726, t43729, t43735, t43737)
}
