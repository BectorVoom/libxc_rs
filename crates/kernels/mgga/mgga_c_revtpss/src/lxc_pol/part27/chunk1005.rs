//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1005/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1005<F: Float>(t114: F, t25824: F, t2339: F, t68: F, t2340: F, t2366: F, t6998: F, t25822: F) -> (F, F) {
    let t115 = 1.0 < t114;
    let t25825 = 2.0 / 3.0 * t25824;
    let t25826 = t68 * t2339;
    let t25827 = t25826 * t2340;
    let t25829 = t6998 * t2366;
    let t25832 = piecewise3(t115, 0.0, t25822 + t25825 + t25827 / 4.0 - t25829 / 8.0);
    (t25826, t25832)
}
