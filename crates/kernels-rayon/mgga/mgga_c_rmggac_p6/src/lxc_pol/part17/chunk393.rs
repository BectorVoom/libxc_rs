//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 393/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk393(t2367: f64, t515: f64, t511: f64, t623: f64, t551: f64, t665: f64, t558: f64, t2295: f64, t793: f64, t2298: f64, t797: f64, t2301: f64, t305: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2368 = t515 * t2367;
    let t2373 = t623 * t511;
    let t2376 = t665 * t551;
    let t2379 = t665 * t558;
    let t2382 = t793 * t2295;
    let t2384 = t797 * t2298;
    let t2386 = t305 * t2301;
    (t2368, t2373, t2376, t2379, t2382, t2384, t2386)
}
