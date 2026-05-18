//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 660/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk660<F: Float>(t187: F, t190: F, t5044: F, t25: F, t4941: F, t4943: F, t4945: F, t4947: F, t4954: F, t4969: F, t4974: F, t4978: F, t5233: F, t5236: F) -> F {
    let t5241 = F::new(0.29629629629629629629e-1) * t190 * t5044 * t187;
    let t5245 = -F::new(0.47988888888888888888e-1) * t4941 + F::new(0.35991666666666666666e-1) * t4947 + F::new(0.23994444444444444444e-1) * t4943 - F::new(0.39990740740740740742e-1) * t4954 - F::new(0.35991666666666666667e-1) * t4978 - F::new(0.39999999999999999999e-1) * t25 * t5233 + F::new(0.39999999999999999999e-1) * t25 * t5236 - t5241 - F::new(0.21595e0) * t4969 + F::new(0.21595e0) * t4974 - F::new(0.71983333333333333333e-1) * t4945;
    t5245
}
