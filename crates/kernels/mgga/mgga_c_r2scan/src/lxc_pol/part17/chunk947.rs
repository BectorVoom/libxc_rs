//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 947/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk947<F: Float>(t10950: F, t10978: F, t10980: F, t10648: F, t2317: F, t3314: F, t3448: F, t2104: F, t2302: F, t2304: F, t10677: F, t57: F, t3439: F, t875: F, t10647: F, t10649: F, t2049: F, t3438: F, t357: F, t6806: F) -> (F, F, F, F, F, F) {
    let t37393 = t10978 * t10980 * t10950;
    let t37397 = t10648 * t3314 * t2317 * t3448;
    let t37400 = t2302 * t2104 * t2304;
    let t37401 = t37400 * t10677;
    let t37403 = t57 * t2304;
    let t37406 = t10978 * t37403 * t875 * t3439;
    let t37412 = t6806 * t357 * t10647 * t10649 * t3438 * t2049;
    (t37393, t37397, t37400, t37401, t37406, t37412)
}
