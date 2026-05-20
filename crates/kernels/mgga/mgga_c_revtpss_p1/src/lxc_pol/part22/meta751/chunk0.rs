//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2824/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2824<F: Float>(t11298: F, t910: F, t41306: F, t3335: F, t11199: F, t988: F, t378: F, t11198: F, t340: F, t338: F, t11119: F, t384: F) -> (F, F, F, F, F, F, F, F, F) {
    let t41883 = t910 * t11298;
    let t41908 = F::cast_from(0.17757530864197530864e0_f64) * t41306;
    let t41936 = t3335 * t3335;
    let t41937 = F::new(1.0) / t41936;
    let t42013 = F::cast_from(0.86419753086419753087e-1_f64) * t41306;
    let t42051 = t988 * t11199;
    let t42052 = t42051 * t378;
    let t42058 = F::new(1.0) / t11198 / t340;
    let t42059 = t338 * t42058;
    let t42060 = t42059 * t378;
    let t42066 = F::new(1.0) / t11119 / t384;
    (t41883, t41908, t41937, t42013, t42051, t42052, t42059, t42060, t42066)
}
