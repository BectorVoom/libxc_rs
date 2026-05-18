//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1078/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1078<F: Float>(t12041: F, t2383: F, t3037: F, t829: F, t830: F, t831: F, t1105: F, t2501: F, t2370: F, t1115: F, t11409: F, t12101: F, t12111: F, t12121: F, t12125: F, t2498: F, t2503: F, t3040: F, t3047: F, t3052: F, t3066: F, t335: F, t827: F, t844: F, t8584: F, t8592: F, t8818: F, t9718: F, t9723: F) -> (F, F) {
    let t12130 = t12041 * t2383;
    let t12133 = t829 * t830 * t831 * t3037;
    let t12136 = t2501 * t1105;
    let t12138 = t2370 * t830 * t12136;
    let t12147 = t3066 * t11409 / F::new(24.0) - t335 * t12101 / F::new(96.0) - F::new(35.0) / F::new(216.0) * t8818 - t1115 * t8592 / F::new(48.0) - t1115 * t9723 / F::new(24.0) + t827 * t12111 / F::new(48.0) - t3040 * t3052 / F::new(24.0) - t2498 * t3052 / F::new(24.0) - t1115 * t9718 / F::new(24.0) - t844 * t12121 / F::new(24.0) - t844 * t12125 / F::new(24.0) + t3040 * t2503 / F::new(48.0) + t12130 * t12133 / F::new(48.0) - t827 * t12138 / F::new(24.0) - t3040 * t3047 / F::new(48.0) - t2498 * t3047 / F::new(48.0) - t1115 * t8584 / F::new(48.0);
    (t12136, t12147)
}
