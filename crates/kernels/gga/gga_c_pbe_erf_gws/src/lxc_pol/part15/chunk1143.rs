//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1143/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1143<F: Float>(t14031: F, t9348: F, t14011: F, t9666: F, t14538: F, t51329: F, t4028: F, t9131: F, t9135: F, t14015: F, t9655: F, t51421: F, t9490: F, t9588: F, t14498: F, t9353: F) -> (F, F, F, F, F, F, F, F, F) {
    let t54162 = t14031 * t9348;
    let t54164 = t14011 * t9666;
    let t54166 = t14538 * t51329;
    let t54167 = 7.0 / 144.0 * t54166;
    let t54168 = t4028 * t9131;
    let t54170 = t4028 * t9135;
    let t54173 = t14015 * t9655;
    let t54175 = t51421 * t9490;
    let t54177 = t14011 * t9588;
    let t54179 = t14498 * t9353;
    (t54162, t54164, t54167, t54168, t54170, t54173, t54175, t54177, t54179)
}
