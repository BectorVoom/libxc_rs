//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2830/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2830<F: Float>(t3181: F, t675: F, t1063: F, t247: F, t2853: F, t283: F, t2852: F, t1025: F, t3218: F, t371: F, t676: F, t11144: F, t3252: F) -> (F, F, F, F, F) {
    let t42447 = t675 * t3181;
    let t42450 = t1063 * t247 * t42447 * t2853;
    let t42471 = F::cast_from(1.0_f64) / t283 / t2852;
    let t42481 = t1025 * t371 * t676 * t3218;
    let t42518 = t3252 * t11144;
    (t42447, t42450, t42471, t42481, t42518)
}
