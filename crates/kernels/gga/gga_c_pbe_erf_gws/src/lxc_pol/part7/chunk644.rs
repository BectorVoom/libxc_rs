//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 644/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk644<F: Float>(t1721: F, t401: F, t1715: F, t25: F, t5022: F, t5025: F, t5030: F, t5034: F, t5039: F, t5042: F, t5047: F, t5049: F, t5052: F) -> F {
    let t5054 = t401 * t1721;
    let t5056 = t401 * t1715;
    let t5058 = -F::cast_from(0.26666666666666666667e-1_f64) * t5022 + F::cast_from(0.13333333333333333333e-1_f64) * t25 * t5025 - F::cast_from(0.66666666666666666666e-2_f64) * t25 * t5030 - F::cast_from(0.39999999999999999999e-1_f64) * t25 * t5034 + F::cast_from(0.39999999999999999999e-1_f64) * t25 * t5039 - F::cast_from(0.71983333333333333333e-1_f64) * t5042 - t5047 - F::cast_from(0.66666666666666666667e-2_f64) * t25 * t5049 - F::cast_from(0.22222222222222222222e-1_f64) * t5052 + F::cast_from(0.13333333333333333334e-1_f64) * t5054 + F::cast_from(0.44444444444444444445e-2_f64) * t5056;
    t5058
}
