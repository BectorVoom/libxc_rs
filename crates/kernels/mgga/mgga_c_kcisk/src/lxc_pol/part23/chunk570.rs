//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 570/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk570<F: Float>(t1163: F, t1322: F, t3937: F, t1390: F, t403: F, t3278: F, t1312: F, t1313: F, t3283: F, t447: F) -> (F, F, F, F, F, F, F) {
    let t3938 = t1163 * t1322;
    let t3939 = t3937 * t3938;
    let t3942 = t403 * t1390;
    let t3943 = t3942 * t3278;
    let t3944 = t1312 * t3943;
    let t3947 = t1313 * t3283;
    let t3948 = t1312 * t3947;
    let t3951 = 1.0 / t447;
    (t3938, t3939, t3943, t3944, t3947, t3948, t3951)
}
