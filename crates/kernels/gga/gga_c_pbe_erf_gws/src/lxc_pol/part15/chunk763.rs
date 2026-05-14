//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 763/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk763<F: Float>(t369: F, t6588: F, t371: F, t364: F, t56: F, t6045: F, t333: F, t338: F, t348: F, t745: F, t814: F, t2129: F, t2142: F, t2123: F, t6183: F, t2120: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6589 = t6588 * t369;
    let t6590 = t6589 * t371;
    let t6592 = 595.0 / 10368.0 * t364 * t6590;
    let t6593 = t6045 * t56;
    let t6594 = t6593 * t333;
    let t6597 = 455.0 / 1296.0 * t348 * t6594 * t338;
    let t6598 = t745 * t814;
    let t6603 = t2129 * t2142;
    let t6605 = t6183 * t2123;
    let t6606 = t2120 * t6605;
    (t6589, t6590, t6592, t6594, t6597, t6598, t6603, t6605, t6606)
}
