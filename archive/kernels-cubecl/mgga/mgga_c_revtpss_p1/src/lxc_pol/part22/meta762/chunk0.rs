//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2844/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2844<F: Float>(t1086: F, t11200: F, t3090: F, t11671: F, t11926: F, t16565: F, t994: F, t42859: F, t42862: F, t342: F, t3145: F, t368: F) -> (F, F, F, F, F, F) {
    let t43291 = t11200 * t1086 * t3090;
    let t43297 = t11926 * t11671;
    let t43341 = t994 * t16565;
    let t43346 = t42859 * t42862;
    let t43347 = t342 * t43346;
    let t43350 = F::cast_from(1.0_f64) / t3145 / t368;
    (t43291, t43297, t43341, t43346, t43347, t43350)
}
