//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3623/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3623<F: Float>(t16682: F, t5192: F, t20652: F, t44012: F, t12227: F, t20651: F, t3427: F, t3385: F, t44091: F, t44093: F, t6438: F, t5219: F, t5412: F) -> (F, F, F, F, F) {
    let t68631 = F::cast_from(0.23392894490538584828e1_f64) * t5192 * t16682;
    let t68633 = F::cast_from(0.1034520258385468006e4_f64) * t44012 * t20652;
    let t68636 = F::cast_from(0.51726012919273400301e3_f64) * t12227 * t20651 * t3427;
    let t68640 = F::cast_from(0.24955700379505800916e5_f64) * t44091 * t6438 * t44093 * t3385;
    let t68658 = t5219 * t5412;
    (t68631, t68633, t68636, t68640, t68658)
}
