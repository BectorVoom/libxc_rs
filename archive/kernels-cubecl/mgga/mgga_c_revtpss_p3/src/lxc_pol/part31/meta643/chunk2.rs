//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2103/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2103<F: Float>(t114: F, t105885: F, t508: F, t651: F, t28166: F, t7897: F, t28168: F, t22287: F, t28167: F, t8996: F, t5824: F, t775: F, t5966: F, t605: F) -> (F, F, F, F, F, F) {
    let t115 = F::cast_from(1.0_f64) < t114;
    let t105886 = piecewise3::<F>(t115, F::cast_from(0.0_f64), t105885);
    let t105889 = F::cast_from(2.0_f64) * t651 * t508 * t105886;
    let t105892 = t7897 * t28166;
    let t105894 = F::cast_from(12.0_f64) * t105892 * t28168;
    let t105897 = F::cast_from(6.0_f64) * t28167 * t8996 * t22287;
    let t105898 = t5824 * t775;
    let t105902 = t605 * t5966;
    (t105886, t105889, t105894, t105897, t105898, t105902)
}
