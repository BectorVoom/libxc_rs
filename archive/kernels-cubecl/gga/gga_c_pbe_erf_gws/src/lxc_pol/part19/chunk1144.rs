//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1144/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1144<F: Float>(t3189: F, t3974: F, t3990: F, t14637: F, t2409: F, t8590: F, t3965: F, t14113: F, t4142: F, t1114: F, t13791: F) -> (F, F, F, F, F, F) {
    let t14639 = t3990 * t3974 * t3189;
    let t14640 = t14637 * t14639;
    let t14648 = t2409 * t8590;
    let t14649 = t3965 * t14648;
    let t14655 = t14113 * t4142;
    let t14657 = t1114 * t13791;
    (t14639, t14640, t14648, t14649, t14655, t14657)
}
