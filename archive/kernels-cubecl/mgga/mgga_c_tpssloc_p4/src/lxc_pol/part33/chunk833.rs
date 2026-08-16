//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 833/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk833<F: Float>(t10478: F, t3128: F, t10472: F, t1015: F, t10277: F, t2978: F, t10213: F, t10216: F, t2775: F, t283: F, t61: F, t2770: F, t976: F) -> (F, F, F, F, F, F) {
    let t10875 = t3128 * t10478;
    let t10876 = t10472 * t10875;
    let t10882 = t1015 * t10478;
    let t10883 = t10472 * t10882;
    let t10930 = t2978 * t10277;
    let t10942 = t10213 * t10216;
    let t10969 = F::cast_from(1.0_f64) / t283 / t2775;
    let t10970 = t61 * t10969;
    let t10996 = t976 * t2770;
    (t10876, t10883, t10930, t10942, t10970, t10996)
}
