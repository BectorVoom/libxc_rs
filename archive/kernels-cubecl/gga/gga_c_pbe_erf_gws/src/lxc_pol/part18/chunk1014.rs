//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1014/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1014<F: Float>(t10013: F, t11310: F, t11312: F, t11314: F, t3772: F, t817: F, t3776: F, t745: F, t1076: F, t2848: F, t3373: F, t10272: F, t2102: F, t2107: F, t3030: F, t3033: F, t323: F, t6089: F, t6096: F, t818: F, t9147: F, t9150: F) -> (F, F) {
    let t11316 = t10013 + t11310 + t11312 + t11314;
    let t11318 = t3772 * t817;
    let t11328 = t3776 * t745;
    let t11331 = t1076 * t2848;
    let t11335 = t3373 * t745;
    let t11339 = -t10272 * t818 - F::cast_from(2.0_f64) * t1076 * t9147 + t11316 * t323 - t11318 * t745 - F::cast_from(6.0_f64) * t11328 * t6096 + F::cast_from(4.0_f64) * t11331 * t2107 + F::cast_from(2.0_f64) * t11335 * t2107 - t2102 * t3373 - F::cast_from(2.0_f64) * t2848 * t3030 + F::cast_from(4.0_f64) * t3033 * t9150 + F::cast_from(2.0_f64) * t3776 * t6089;
    (t11316, t11339)
}
