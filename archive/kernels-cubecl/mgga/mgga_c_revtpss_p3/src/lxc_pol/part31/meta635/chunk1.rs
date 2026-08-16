//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2090/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2090<F: Float>(t7150: F, t99708: F, t1977: F, t994: F, t11627: F, t1983: F, t99682: F, t11223: F, t7143: F, t3057: F, t7810: F, t11120: F) -> (F, F, F, F, F, F) {
    let t100494 = t7150 * t99708;
    let t100586 = t994 * t1977;
    let t100596 = t1983 * t99682 * t11627;
    let t100658 = t11223 * t7143;
    let t100681 = t3057 * t7810;
    let t100690 = t7143 * t11120;
    (t100494, t100586, t100596, t100658, t100681, t100690)
}
