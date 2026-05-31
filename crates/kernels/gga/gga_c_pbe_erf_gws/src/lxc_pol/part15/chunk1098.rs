//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1098/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1098<F: Float>(t13972: F, t3993: F, t13921: F, t13925: F, t13930: F, t13939: F, t13945: F, t13948: F, t13950: F, t13955: F, t13958: F, t13962: F, t13965: F, t13966: F, t13969: F, t2384: F, t2388: F, t2392: F, t4002: F, t4385: F, t6793: F, t827: F) -> (F, F) {
    let t13973 = t13972 * t3993;
    let t13974 = F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t13973;
    let t13975 = -t13921 / F::cast_from(768.0_f64) + t4385 * t13925 / F::cast_from(96.0_f64) + t6793 * t13930 / F::cast_from(24.0_f64) - t2388 * t4002 / F::cast_from(96.0_f64) - t2392 * t4002 / F::cast_from(96.0_f64) - t827 * t13939 / F::cast_from(48.0_f64) + t13945 / F::cast_from(96.0_f64) - t13948 - t13950 / F::cast_from(24.0_f64) + t13955 - t13958 / F::cast_from(768.0_f64) - t2384 * t4002 / F::cast_from(96.0_f64) + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t13962 + t13965 + t13966 / F::cast_from(24.0_f64) - t13969 / F::cast_from(48.0_f64) + t13974;
    (t13973, t13975)
}
