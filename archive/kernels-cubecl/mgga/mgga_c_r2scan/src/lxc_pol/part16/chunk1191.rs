//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1191/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1191<F: Float>(t37982: F, t9373: F, t11654: F, t7601: F, t10743: F, t3198: F, t261: F, t3299: F, t9451: F, t31060: F, t3333: F, t12533: F, t22796: F) -> (F, F, F, F, F, F) {
    let t43230 = t37982 * t9373;
    let t43232 = t7601 * t11654;
    let t43234 = t10743 * t3198;
    let t43238 = t3299 * t261 * t9451;
    let t43240 = t31060 * t3333;
    let t43242 = t22796 * t12533;
    (t43230, t43232, t43234, t43238, t43240, t43242)
}
