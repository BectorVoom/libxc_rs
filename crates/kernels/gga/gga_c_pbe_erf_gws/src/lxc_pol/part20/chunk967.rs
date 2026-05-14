//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 967/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk967<F: Float>(t1105: F, t2501: F, t2370: F, t830: F, t1115: F, t11409: F, t12101: F, t12111: F, t12121: F, t12125: F, t12130: F, t12133: F, t2498: F, t2503: F, t3040: F, t3047: F, t3052: F, t3066: F, t335: F, t827: F, t844: F, t8584: F, t8592: F, t8818: F, t9718: F, t9723: F) -> (F, F) {
    let t12136 = t2501 * t1105;
    let t12138 = t2370 * t830 * t12136;
    let t12147 = t3066 * t11409 / 24.0 - t335 * t12101 / 96.0 - 35.0 / 216.0 * t8818 - t1115 * t8592 / 48.0 - t1115 * t9723 / 24.0 + t827 * t12111 / 48.0 - t3040 * t3052 / 24.0 - t2498 * t3052 / 24.0 - t1115 * t9718 / 24.0 - t844 * t12121 / 24.0 - t844 * t12125 / 24.0 + t3040 * t2503 / 48.0 + t12130 * t12133 / 48.0 - t827 * t12138 / 24.0 - t3040 * t3047 / 48.0 - t2498 * t3047 / 48.0 - t1115 * t8584 / 48.0;
    (t12136, t12147)
}
