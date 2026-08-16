//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 754/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk754<F: Float>(t2137: F, t6183: F, t2134: F, t2132: F, t2271: F, t822: F, t2138: F, t2113: F, t2255: F, t2313: F, t2081: F, t4394: F) -> (F, F, F, F, F, F, F) {
    let t6184 = t6183 * t2137;
    let t6185 = t2134 * t6184;
    let t6186 = F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t6185;
    let t6187 = t2271 * t2132;
    let t6188 = t822 * t6187;
    let t6190 = t6188 * t2138 / F::cast_from(32.0_f64);
    let t6192 = t2255 * t2113 * t2313;
    let t6195 = t2081 * t4394;
    (t6184, t6186, t6187, t6188, t6190, t6192, t6195)
}
