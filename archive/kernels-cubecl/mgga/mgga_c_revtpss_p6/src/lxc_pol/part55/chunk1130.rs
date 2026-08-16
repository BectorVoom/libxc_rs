//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1130/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1130<F: Float>(t120002: F, t240: F, t27: F, t822: F, t119967: F, t119837: F, t14686: F, t837: F, t119833: F, t814: F, t853: F, t802: F) -> (F, F, F, F, F, F, F) {
    let t120003 = F::cast_from(0.66119071333692697238e-4_f64) * t120002;
    let t120010 = t822 * t27 * t240;
    let t120011 = t119967 * t120010;
    let t120013 = t14686 * t119837 * t837;
    let t120014 = t120011 * t120013;
    let t120016 = t119833 * t120010;
    let t120017 = t120016 * t120013;
    let t120042 = t814 * t853;
    let t120043 = t120042 * t802;
    (t120003, t120011, t120014, t120016, t120017, t120042, t120043)
}
