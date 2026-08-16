//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 961/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk961<F: Float>(t40458: F, t35705: F, t35707: F, t35713: F, t35717: F, t35720: F, t35724: F, t35729: F, t40414: F, t40420: F, t40425: F, t40431: F, t40437: F, t40442: F, t40448: F, t40451: F, t40456: F) -> F {
    let t40459 = F::cast_from(0.79828278012425390426e-1_f64) * t40458;
    let t40463 = -F::cast_from(0.70441376091769752086e-2_f64) * t35705 + F::cast_from(0.1064114997332445985e-4_f64) * t40414 + F::cast_from(0.53205749866622299248e-5_f64) * t40420 + F::cast_from(0.53205749866622299248e-5_f64) * t40425 - F::cast_from(0.85129199786595678796e-5_f64) * t40431 - F::cast_from(0.85129199786595678796e-5_f64) * t40437 + F::cast_from(0.25538759935978703638e-4_f64) * t40442 - F::cast_from(0.25538759935978703638e-4_f64) * t40448 + F::cast_from(0.85129199786595678796e-5_f64) * t40451 - F::cast_from(0.1064114997332445985e-4_f64) * t40456 - t40459 + F::cast_from(0.60975299583150056628e-3_f64) * t35707 + t35713 + t35717 - F::cast_from(0.86737941314158990624e-4_f64) * t35720 - F::cast_from(0.86737941314158990624e-4_f64) * t35724 - t35729;
    t40463
}
