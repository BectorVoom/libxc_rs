//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 934/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk934<F: Float>(t42058: F, t8512: F, t31991: F, t378: F, t120190: F, t32009: F, t31883: F, t31909: F, t7165: F, t31999: F, t8513: F, t93488: F, t1982: F, t31926: F, t3268: F, t31927: F, t994: F) -> (F, F, F, F, F, F, F) {
    let t120471 = t8512 * t42058;
    let t120473 = t120471 * t378 * t31991;
    let t120476 = t32009 * t120190;
    let t120479 = t31909 * t31883;
    let t120481 = t120471 * t7165;
    let t120495 = t8513 * t93488 * t31999;
    let t120507 = t1982 * t31926 * t3268;
    let t120513 = t994 * t31927;
    (t120473, t120476, t120479, t120481, t120495, t120507, t120513)
}
