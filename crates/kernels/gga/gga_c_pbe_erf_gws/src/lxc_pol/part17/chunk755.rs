//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 755/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk755<F: Float>(t1703: F, t395: F, t1693: F, t1639: F, t56: F, t1672: F, t662: F, t211: F, t1794: F, t582: F, t648: F, t618: F) -> (F, F, F, F, F, F, F) {
    let t5085 = t395 * t1703;
    let t5087 = t395 * t1693;
    let t5089 = t56 * t1639;
    let t5102 = t1672 * t662;
    let t5103 = t211 * t5102;
    let t5105 = t582 * t1794;
    let t5106 = t211 * t5105;
    let t5108 = t648 * t648;
    let t5109 = F::cast_from(1.0_f64) / t5108;
    let t5116 = t1672 * t618;
    (t5085, t5087, t5089, t5103, t5106, t5109, t5116)
}
