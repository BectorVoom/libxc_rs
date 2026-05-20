//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1354/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1354<F: Float>(t41741: F, t315: F, t41224: F, t41306: F, t3335: F, t11198: F, t340: F, t338: F, t378: F, t11119: F, t384: F, t225: F) -> (F, F, F, F, F, F, F, F) {
    let t41742 = F::new(1.0) / t41741;
    let t41759 = t315 * t41224;
    let t41908 = F::cast_from(0.17757530864197530864e0_f64) * t41306;
    let t41936 = t3335 * t3335;
    let t41937 = F::new(1.0) / t41936;
    let t42013 = F::cast_from(0.86419753086419753087e-1_f64) * t41306;
    let t42058 = F::new(1.0) / t11198 / t340;
    let t42059 = t338 * t42058;
    let t42060 = t42059 * t378;
    let t42066 = F::new(1.0) / t11119 / t384;
    let t42067 = t225 * t42066;
    (t41742, t41759, t41908, t41937, t42013, t42059, t42060, t42067)
}
