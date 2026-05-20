//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2007/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2007<F: Float>(t25894: F, t94394: F, t25945: F, t9285: F, t25944: F, t2482: F, t7262: F, t814: F, t820: F, t844: F, t596: F, t7269: F) -> (F, F, F, F, F, F) {
    let t94395 = t25894 * t94394;
    let t94407 = t25945 * t9285;
    let t94409 = F::cast_from(0.68540937416128198417e-2_f64) * t25944 * t94407;
    let t94423 = t2482 * t7262 * t814;
    let t94429 = t820 * t7262 * t844;
    let t94443 = t2482 * t7269 * t596;
    (t94395, t94407, t94409, t94423, t94429, t94443)
}
