//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 796/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk796<F: Float>(t22: F, t4258: F, t191: F, t369: F, t371: F, t364: F, t56: F, t6045: F, t333: F, t338: F, t348: F, t745: F, t814: F) -> (F, F, F, F, F, F, F) {
    let t6587 = F::new(1.0) / t22 / t4258;
    let t6588 = t6587 * t191;
    let t6589 = t6588 * t369;
    let t6590 = t6589 * t371;
    let t6592 = F::new(595.0) / F::new(10368.0) * t364 * t6590;
    let t6593 = t6045 * t56;
    let t6594 = t6593 * t333;
    let t6597 = F::new(455.0) / F::new(1296.0) * t348 * t6594 * t338;
    let t6598 = t745 * t814;
    (t6587, t6588, t6592, t6593, t6594, t6597, t6598)
}
