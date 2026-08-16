//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 392/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk392(t2295: f64, t793: f64, t2298: f64, t797: f64, t2301: f64, t305: f64, t2068: f64, t2353: f64, t2073: f64, t2356: f64, t36: f64, t570: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2382 = t793 * t2295;
    let t2384 = t797 * t2298;
    let t2386 = t305 * t2301;
    let t2388 = t2068 * t2353;
    let t2390 = t2073 * t2356;
    let t2392 = t36 * t570;
    (t2382, t2384, t2386, t2388, t2390, t2392)
}
