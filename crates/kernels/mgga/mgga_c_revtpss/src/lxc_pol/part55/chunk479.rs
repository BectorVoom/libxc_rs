//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 479/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk479<F: Float>(t360: F, t365: F, t1038: F, t72: F, t1087: F, t1066: F, t828: F, t1043: F, t73: F, t357: F, t905: F, t606: F) -> (F, F, F, F, F, F, F) {
    let t3088 = t360 * t365;
    let t3089 = t1038 * t72;
    let t3090 = t3088 * t3089;
    let t3091 = t1087 * t3090;
    let t3092 = t828 * t1066;
    let t3093 = t1043 * t73;
    let t3094 = t357 * t905;
    let t3095 = t3094 * t606;
    (t3088, t3089, t3090, t3091, t3092, t3093, t3095)
}
