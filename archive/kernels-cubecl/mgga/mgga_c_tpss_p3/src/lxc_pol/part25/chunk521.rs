//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 521/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk521<F: Float>(t655: F, t130: F, t146: F, t2289: F) -> (F, F, F, F, F, F, F) {
    let t2303 = t655 * t655;
    let t2304 = F::cast_from(1.0_f64) / t2303;
    let t2305 = t130 * t2304;
    let t2306 = t146 * t146;
    let t2307 = F::cast_from(1.0_f64) / t2306;
    let t2308 = t2289 * t2307;
    let t2310 = F::cast_from(0.16081979498692535067e2_f64) * t2305 * t2308;
    (t2303, t2304, t2305, t2306, t2307, t2308, t2310)
}
