//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1095/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1095<F: Float>(t1150: F, t51200: F, t14028: F, t3295: F, t14024: F, t3113: F, t3123: F, t51430: F, t14538: F, t51329: F, t3299: F, t2127: F, t3258: F, t850: F, t14046: F, t14522: F) -> (F, F, F, F, F, F, F, F) {
    let t54126 = t51200 * t1150;
    let t54128 = t14028 * t3295;
    let t54135 = t3113 * t14024;
    let t54152 = t3123 * t51430;
    let t54166 = t14538 * t51329;
    let t54198 = t14028 * t3299;
    let t54230 = t850 * t3258 * t2127;
    let t54236 = t14046 * t14522;
    (t54126, t54128, t54135, t54152, t54166, t54198, t54230, t54236)
}
