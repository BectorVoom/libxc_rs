//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 930/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk930<F: Float>(t5796: F, t5814: F, t5816: F, t102: F, t1504: F, t978: F, t5825: F, t967: F, t120: F, t8102: F, t506: F, t1243: F, t2863: F) -> (F, F, F, F, F, F, F, F) {
    let t8177 = F::cast_from(0.6495611111111111111e0_f64) * t5796;
    let t8181 = F::cast_from(0.97434166666666666666e0_f64) * t5814;
    let t8182 = F::cast_from(0.12991222222222222222e1_f64) * t5816;
    let t8186 = F::cast_from(0.1753815e2_f64) * t102 * t978 * t1504;
    let t8187 = t5825 * t967;
    let t8191 = t120 * t8102;
    let t8193 = F::cast_from(0.2923025e1_f64) * t102 * t8191;
    let t8194 = t506 * t8102;
    let t8197 = t2863 * t1243;
    (t8177, t8181, t8182, t8186, t8187, t8193, t8194, t8197)
}
