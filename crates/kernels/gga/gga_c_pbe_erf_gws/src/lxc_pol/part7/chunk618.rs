//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 618/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk618<F: Float>(t4972: F, t571: F, t11: F, t4360: F, t572: F, t4940: F, t4941: F, t4943: F, t4945: F, t4947: F, t4954: F, t4960: F, t4965: F, t4969: F, t173: F, t184: F) -> (F, F, F, F, F, F, F, F) {
    let t4973 = t571 * t4972;
    let t4974 = t11 * t4973;
    let t4976 = t572 * t4360;
    let t4977 = t571 * t4976;
    let t4978 = t11 * t4977;
    let t4980 = t4940 + 0.25188888888888888889e-2 * t4941 - 0.12594444444444444445e-2 * t4943 + 0.37783333333333333335e-2 * t4945 - 0.18891666666666666667e-2 * t4947 + 0.20990740740740740742e-2 * t4954 - 0.75566666666666666669e-2 * t4960 + 0.37783333333333333335e-2 * t4965 + 0.11335e-1 * t4969 - 0.11335e-1 * t4974 + 0.18891666666666666667e-2 * t4978;
    let t4981 = t173 * t4980;
    let t4982 = t4981 * t184;
    (t4973, t4974, t4976, t4977, t4978, t4980, t4981, t4982)
}
