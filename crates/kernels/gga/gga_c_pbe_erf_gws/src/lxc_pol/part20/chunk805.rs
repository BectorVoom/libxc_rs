//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 805/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk805<F: Float>(t2251: F, t2299: F, t2276: F, t22: F, t4258: F, t191: F, t369: F, t371: F, t364: F, t56: F, t6045: F, t333: F) -> (F, F, F, F, F) {
    let t6578 = t2251 * t2299;
    let t6579 = t2276 * t6578;
    let t6587 = F::cast_from(1.0_f64) / t22 / t4258;
    let t6588 = t6587 * t191;
    let t6589 = t6588 * t369;
    let t6590 = t6589 * t371;
    let t6592 = F::cast_from(595.0_f64) / F::cast_from(10368.0_f64) * t364 * t6590;
    let t6593 = t6045 * t56;
    let t6594 = t6593 * t333;
    (t6579, t6589, t6590, t6592, t6594)
}
