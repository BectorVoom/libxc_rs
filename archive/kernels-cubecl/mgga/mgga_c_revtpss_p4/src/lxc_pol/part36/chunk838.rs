//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 838/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk838<F: Float>(t10985: F, t2454: F, t252: F, t2769: F, t786: F, t866: F, t225: F, t788: F, t9288: F, t787: F, t781: F, t9292: F) -> (F, F, F, F, F, F, F, F) {
    let t10987 = F::cast_from(0.46263278077393568556e-2_f64) * t2454 * t10985;
    let t10994 = t252 * t2769;
    let t10995 = t786 * t10994;
    let t11006 = t866 * t866;
    let t11007 = F::cast_from(1.0_f64) / t11006;
    let t11008 = t225 * t11007;
    let t11015 = t788 * t9288;
    let t11017 = F::cast_from(0.30356481678079769392e-1_f64) * t787 * t11015;
    let t11040 = F::cast_from(0.17073386770573548589e-1_f64) * t9292 * t781;
    (t10987, t10995, t11006, t11007, t11008, t11015, t11017, t11040)
}
