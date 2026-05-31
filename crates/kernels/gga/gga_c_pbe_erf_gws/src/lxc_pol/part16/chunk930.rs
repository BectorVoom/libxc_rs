//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 930/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk930<F: Float>(t2897: F, t501: F, t395: F, t1552: F, t978: F, t1251: F, t1563: F, t2873: F, t102: F, t2885: F, t481: F, t1533: F, t974: F) -> (F, F, F, F, F) {
    let t8156 = t501 * t2897;
    let t8158 = F::cast_from(0.146904e1_f64) * t8156 * t395;
    let t8159 = t1552 * t978;
    let t8160 = t8159 * t1251;
    let t8162 = t1563 * t2873;
    let t8171 = F::cast_from(0.116921e2_f64) * t102 * t2885 * t481;
    let t8174 = F::cast_from(0.584605e1_f64) * t102 * t974 * t1533;
    (t8158, t8160, t8162, t8171, t8174)
}
