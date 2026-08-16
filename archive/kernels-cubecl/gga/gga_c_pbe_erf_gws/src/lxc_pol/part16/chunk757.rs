//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 757/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk757<F: Float>(t5116: F, t616: F, t1651: F, t197: F, t1823: F, t1820: F, t597: F, t1828: F, t587: F, t1630: F, t649: F, t1816: F) -> (F, F, F, F, F, F, F) {
    let t5117 = t616 * t5116;
    let t5125 = t1651 * t197;
    let t5126 = t5125 * t1823;
    let t5127 = t1820 * t5126;
    let t5129 = t1651 * t597;
    let t5130 = t5129 * t1828;
    let t5131 = t587 * t5130;
    let t5137 = t1630 * t649;
    let t5138 = t5137 * t1816;
    (t5117, t5125, t5127, t5129, t5131, t5137, t5138)
}
