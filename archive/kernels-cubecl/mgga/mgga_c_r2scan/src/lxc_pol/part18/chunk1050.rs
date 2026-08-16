//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1050/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1050<F: Float>(t37386: F, t10950: F, t10978: F, t10980: F, t10648: F, t2317: F, t3314: F, t3448: F, t2104: F, t2302: F, t2304: F, t10677: F) -> (F, F, F, F, F) {
    let t37387 = F::cast_from(0.10260057759007034251e-5_f64) * t37386;
    let t37393 = t10978 * t10980 * t10950;
    let t37397 = t10648 * t3314 * t2317 * t3448;
    let t37398 = F::cast_from(0.24390119833260022651e-2_f64) * t37397;
    let t37400 = t2302 * t2104 * t2304;
    let t37401 = t37400 * t10677;
    (t37387, t37393, t37398, t37400, t37401)
}
