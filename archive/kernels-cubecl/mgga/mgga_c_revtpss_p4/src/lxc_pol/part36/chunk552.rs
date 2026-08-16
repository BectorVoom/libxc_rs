//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 552/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk552<F: Float>(t1247: F, t5265: F, t1263: F, t3367: F, t1032: F, t1770: F, t1246: F, t1774: F, t1038: F, t1802: F, t1244: F, t1241: F) -> (F, F, F, F, F, F, F, F) {
    let t5266 = t1247 * t5265;
    let t5268 = t1263 * t3367;
    let t5273 = t1770 * t1032;
    let t5274 = t5273 * t1246;
    let t5277 = t1263 * t1774;
    let t5291 = t1802 * t1038;
    let t5292 = t1244 * t5291;
    let t5293 = t1241 * t5292;
    (t5266, t5268, t5273, t5274, t5277, t5291, t5292, t5293)
}
