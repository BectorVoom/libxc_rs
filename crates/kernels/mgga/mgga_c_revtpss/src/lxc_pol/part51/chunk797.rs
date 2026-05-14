//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 797/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk797<F: Float>(t5: F, t28115: F, t28157: F, t117: F, t7239: F, t7898: F, t197: F, t530: F, t2013: F, t5627: F, t8996: F, t1310: F, t1453: F, t28050: F, t28053: F, t28058: F, t28060: F, t28062: F, t28065: F, t28069: F, t4248: F, t508: F, t649: F, t651: F, t7007: F, t7725: F, t7883: F, t7894: F) -> (F, F, F, F, F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t28159 = piecewise3(t8, 0.0, t28115 + t28157);
    let t28160 = t28159 * t117;
    let t28165 = 3.0 * t7898 * t7239;
    let t28166 = t197 * t530;
    let t28167 = t2013 * t28166;
    let t28168 = t8996 * t5627;
    let t28170 = 6.0 * t28167 * t28168;
    let t28171 = -t1310 * t7725 + t1453 * t7894 - 2.0 * t28050 * t651 - 2.0 * t28053 * t651 - t28160 * t508 - 2.0 * t4248 * t7007 - t649 * t7883 - t28058 - t28060 - t28062 - t28065 - t28069 + t28165 + t28170;
    (t28159, t28160, t28166, t28167, t28168, t28171)
}
