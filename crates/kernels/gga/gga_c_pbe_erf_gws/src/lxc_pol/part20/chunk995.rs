//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 995/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk995<F: Float>(t10661: F, t10668: F, t10670: F, t10674: F, t10678: F, t10683: F, t10687: F, t5929: F, t5933: F, t5938: F, t5940: F, t5944: F, t7526: F, t7532: F, t8439: F, t8440: F) -> F {
    let t11211 = t10661 + t10668 - t10670 + t7526 - t7532 + t10674 - t10678 + t10683 - t10687 + t5929 + t5933 + F::cast_from(0.21642082724729686754e0_f64) * t5938 + F::cast_from(0.72140275749098955847e-1_f64) * t5940 - t5944 + t8439 + F::new(16.0) / F::new(3.0) * t8440;
    t11211
}
