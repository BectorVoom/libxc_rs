//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 857/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk857<F: Float>(t657: F, t7205: F, t1714: F, t7093: F, t7257: F, t190: F, t25: F, t2718: F, t5044: F, t5047: F, t5082: F, t7269: F, t7272: F, t7274: F, t7279: F, t7280: F, t7285: F, t7288: F, t7290: F, t7291: F, t7294: F, t7297: F, t7300: F) -> F {
    let t7303 = t657 * t7205;
    let t7306 = t1714 * t7093;
    let t7309 = t657 * t7257;
    let t7312 = -F::new(0.15996296296296296296e-1) * t7269 - F::new(0.26393888888888888889e0) * t7272 + F::new(0.13333333333333333333e-1) * t190 * t5044 * t7274 - t5047 - t5082 + t7279 - F::new(0.47988888888888888889e-1) * t7280 - F::new(0.39990740740740740742e-1) * t7285 - t7288 + t7290 - F::new(0.22222222222222222222e-2) * t25 * t7291 - F::new(0.29629629629629629629e-2) * t25 * t7294 + F::new(0.88888888888888888887e-2) * t2718 * t7297 + F::new(0.13333333333333333333e-1) * t25 * t7300 - F::new(0.53333333333333333332e-1) * t2718 * t7303 + F::new(0.13333333333333333333e-1) * t25 * t7306 - F::new(0.39999999999999999999e-1) * t25 * t7309;
    t7312
}
