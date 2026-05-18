//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 516/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk516<F: Float>(t1216: F, t2372: F, t1268: F, t2359: F, t2363: F, t2369: F, t295: F, t305: F, t803: F, t811: F, t991: F, t997: F) -> (F, F) {
    let t2373 = t2372 * t1216;
    let t2376 = -F::new(25.0) / F::new(9.0) * t803 * t991 + F::new(10.0) / F::new(9.0) * t295 * t2359 + F::new(5.0) / F::new(3.0) * t295 * t2363 - F::new(25.0) / F::new(9.0) * t997 * t811 + F::new(10.0) / F::new(9.0) * t305 * t2369 - F::new(5.0) / F::new(3.0) * t305 * t2373 - t1268;
    (t2373, t2376)
}
