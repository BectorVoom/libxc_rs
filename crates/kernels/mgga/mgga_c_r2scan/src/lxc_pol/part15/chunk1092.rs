//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1092/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1092<F: Float>(t2262: F, t4176: F, t3270: F, t1053: F, t10648: F, t10993: F, t6876: F, t58: F, t423: F, t2315: F, t597: F, t10680: F) -> (F, F, F, F, F) {
    let t38288 = t4176 * t2262;
    let t38289 = t3270 * t38288;
    let t38297 = t10648 * t1053 * t6876 * t10993;
    let t38298 = F::new(0.91462949374725084942e-3) * t38297;
    let t38299 = t6876 * t58;
    let t38300 = t38299 * t423;
    let t38301 = t597 * t2315;
    let t38303 = t10680 * t38300 * t38301;
    (t38289, t38298, t38299, t38301, t38303)
}
