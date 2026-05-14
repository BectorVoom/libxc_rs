//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 942/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk942<F: Float>(t19790: F, t910: F, t146: F, t5094: F, t774: F, t560: F, t7977: F, t481: F, t1234: F, t2841: F, t537: F, t7194: F, t113: F, t24165: F, t24118: F, t2185: F, t921: F) -> (F, F, F, F, F, F, F, F, F) {
    let t24916 = t19790 * t910;
    let t25169 = t146 * t5094 * t774;
    let t25172 = t7977 * t560;
    let t25177 = t7977 * t481;
    let t25183 = t2841 * t1234;
    let t25191 = t537 * t7194;
    let t25192 = t25191 * t113;
    let t25303 = t24165 * t113;
    let t25307 = t24118 * t113;
    let t25314 = t921 * t2185;
    (t24916, t25169, t25172, t25177, t25183, t25192, t25303, t25307, t25314)
}
