//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 920/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk920<F: Float>(t10628: F, t549: F, t6111: F, t24505: F, t2684: F, t9438: F, t3271: F, t8634: F, t10050: F, t3040: F, t1457: F, t2103: F, t43213: F) -> (F, F, F, F, F) {
    let t43715 = t6111 * t549 * t10628;
    let t43716 = F::cast_from(0.11916829983950142223e0_f64) * t43715;
    let t43718 = t2684 * t9438 * t24505;
    let t43719 = F::cast_from(0.7988109573733489516e-1_f64) * t43718;
    let t43721 = F::cast_from(0.35750489951850426669e0_f64) * t3271 * t8634;
    let t43723 = F::cast_from(0.35750489951850426669e0_f64) * t10050 * t3040;
    let t43726 = F::cast_from(0.71500979903700853338e0_f64) * t2103 * t1457 * t43213;
    (t43716, t43719, t43721, t43723, t43726)
}
