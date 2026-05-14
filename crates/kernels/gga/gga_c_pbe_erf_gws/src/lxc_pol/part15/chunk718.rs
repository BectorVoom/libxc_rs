//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 718/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk718<F: Float>(t1816: F, t5137: F, t639: F, t1730: F, t1798: F, t1734: F, t582: F, t616: F, t596: F, t1617: F, t732: F, t1672: F, t611: F, t185: F, t108: F, t615: F) -> (F, F, F, F, F, F, F) {
    let t5138 = t5137 * t1816;
    let t5139 = t639 * t5138;
    let t5169 = t1730 * t1798;
    let t5171 = t582 * t1734;
    let t5172 = t616 * t5171;
    let t5174 = t596 * t596;
    let t5175 = 1.0 / t5174;
    let t5205 = t732 * t1617;
    let t5207 = t1672 * t611;
    let t5208 = t185 * t5207;
    let t5210 = t615 * t108;
    (t5139, t5169, t5172, t5175, t5205, t5208, t5210)
}
