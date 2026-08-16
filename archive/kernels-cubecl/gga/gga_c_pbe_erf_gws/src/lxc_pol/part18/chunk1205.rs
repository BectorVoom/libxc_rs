//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1205/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1205<F: Float>(t1109: F, t810: F, t2306: F, t35187: F, t3074: F, t831: F, t9807: F, t2395: F, t3717: F, t1144: F, t858: F, t1105: F, t8749: F) -> (F, F, F, F, F, F) {
    let t35207 = t1109 * t810;
    let t35259 = t2306 * t35187;
    let t35260 = t3074 * t35259;
    let t35428 = t831 * t9807;
    let t35433 = t2395 * t3717;
    let t35566 = t858 * t1144;
    let t35654 = t8749 * t1105;
    (t35207, t35260, t35428, t35433, t35566, t35654)
}
