//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 905/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk905<F: Float>(t1224: F, t3362: F, t10356: F, t1012: F, t1226: F, t697: F, t1222: F, t140: F, t3688: F, t3700: F, t12268: F, t3698: F, t3367: F, t404: F, t12256: F, t1204: F, t3140: F) -> (F, F, F, F, F, F, F) {
    let t13006 = t1224 * t3362;
    let t13007 = t13006 * t10356;
    let t13008 = t1012 * t13007;
    let t13011 = t697 * t1226;
    let t13012 = t1222 * t13011;
    let t13014 = t140 * t3688;
    let t13015 = t1222 * t13014;
    let t13017 = t140 * t3700;
    let t13018 = t1222 * t13017;
    let t13020 = t3698 * t12268;
    let t13021 = t13020 * t10356;
    let t13022 = t1012 * t13021;
    let t13026 = 1.0 / t404 / t3367;
    let t13027 = t13026 * t12256;
    let t13028 = t13027 * t10356;
    let t13029 = t1012 * t13028;
    let t13032 = t1204 * t3140;
    (t13008, t13012, t13015, t13018, t13022, t13029, t13032)
}
