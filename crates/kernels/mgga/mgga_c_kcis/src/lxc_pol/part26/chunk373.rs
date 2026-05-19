//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 373/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk373<F: Float>(t2243: F, t541: F, t303: F, t2237: F, t2239: F, t589: F, t570: F, t573: F, t1395: F, t585: F) -> (F, F, F, F, F, F, F) {
    let t2244 = t541 * t2243;
    let t2245 = t303 * t2244;
    let t2247 = -F::cast_from(0.69505208333333333333e-3_f64) * t2237 * t2239 + F::cast_from(0.24872916666666666666e-2_f64) * t2245;
    let t2248 = t2247 * t589;
    let t2249 = t570 * t573;
    let t2251 = t1395 * t585;
    let t2253 = t2249 / F::new(16.0) - t2251 / F::new(128.0);
    (t2244, t2245, t2247, t2248, t2249, t2251, t2253)
}
