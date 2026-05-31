//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 515/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk515<F: Float>(t2182: F, t339: F, t2074: F, t2100: F, t2178: F, t2181: F, t340: F, t870: F, t871: F, t343: F) -> (F, F, F, F) {
    let t2183 = t339 * t2182;
    let t2186 = t339 * t2074;
    let t2189 = -t2100 * t339 * t340 + F::cast_from(6.0_f64) * t2178 * t871 - F::cast_from(12.0_f64) * t2181 * t2183 + F::cast_from(3.0_f64) * t2186 * t870;
    let t2190 = t2189 * t343;
    (t2183, t2186, t2189, t2190)
}
