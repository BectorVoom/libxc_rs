//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 936/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk936<F: Float>(t7239: F, t7898: F, t197: F, t530: F, t2013: F, t5627: F, t8996: F, t531: F, t7933: F, t7238: F, t2014: F, t1450: F, t5591: F) -> (F, F, F, F, F, F, F, F) {
    let t28165 = F::cast_from(3.0_f64) * t7898 * t7239;
    let t28166 = t197 * t530;
    let t28167 = t2013 * t28166;
    let t28168 = t8996 * t5627;
    let t28170 = F::cast_from(6.0_f64) * t28167 * t28168;
    let t28172 = t531 * t7933;
    let t28173 = t28172 * t7238;
    let t28175 = F::cast_from(3.0_f64) * t2014 * t28173;
    let t28176 = t1450 * t5591;
    (t28165, t28166, t28167, t28168, t28170, t28173, t28175, t28176)
}
