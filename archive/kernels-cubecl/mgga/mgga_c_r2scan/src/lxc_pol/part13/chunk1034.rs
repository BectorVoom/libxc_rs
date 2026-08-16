//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1034/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1034<F: Float>(t537: F, t7194: F, t113: F, t24165: F, t24118: F, t2185: F, t921: F, t19790: F, t1553: F, t7338: F, t2654: F, t6212: F) -> (F, F, F, F, F, F, F) {
    let t25191 = t537 * t7194;
    let t25192 = t25191 * t113;
    let t25303 = t24165 * t113;
    let t25307 = t24118 * t113;
    let t25314 = t921 * t2185;
    let t25397 = t19790 * t921;
    let t25466 = t7338 * t1553;
    let t25480 = t6212 * t2654;
    (t25192, t25303, t25307, t25314, t25397, t25466, t25480)
}
