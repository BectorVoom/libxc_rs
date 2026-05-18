//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1177/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1177<F: Float>(t13781: F, t15145: F, t3972: F, t1109: F, t2118: F, t1113: F, t3975: F, t1192: F, t3886: F, t2409: F, t3067: F, t1076: F, t331: F) -> (F, F, F, F, F, F, F, F) {
    let t15146 = t13781 * t15145;
    let t15147 = t3972 * t15146;
    let t15149 = t2118 * t1109;
    let t15150 = t1113 * t15149;
    let t15151 = t3975 * t15150;
    let t15152 = t3972 * t15151;
    let t15154 = t1192 * t3886;
    let t15156 = t2409 * t3067 * t15154;
    let t15159 = t1076 * t331;
    (t15146, t15147, t15149, t15151, t15152, t15154, t15156, t15159)
}
