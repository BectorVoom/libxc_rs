//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 986/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk986<F: Float>(t43718: F, t3271: F, t8634: F, t10050: F, t3040: F, t1457: F, t2103: F, t43213: F, t43217: F, t43001: F, t10867: F, t9989: F) -> (F, F, F, F, F, F, F) {
    let t43719 = F::new(0.7988109573733489516e-1) * t43718;
    let t43721 = F::new(0.35750489951850426669e0) * t3271 * t8634;
    let t43723 = F::new(0.35750489951850426669e0) * t10050 * t3040;
    let t43726 = F::new(0.71500979903700853338e0) * t2103 * t1457 * t43213;
    let t43729 = F::new(0.71500979903700853338e0) * t2103 * t1457 * t43217;
    let t43731 = t2103 * t1457 * t43001;
    let t43735 = F::new(0.25025342966295298669e1) * t10867 * t1457 * t9989;
    (t43719, t43721, t43723, t43726, t43729, t43731, t43735)
}
