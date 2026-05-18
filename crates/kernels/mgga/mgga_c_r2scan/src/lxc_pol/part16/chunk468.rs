//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 468/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk468<F: Float>(t1010: F, t826: F, t1288: F, t2359: F, t2363: F, t2369: F, t2373: F, t313: F) -> (F, F) {
    let t2381 = t1010 * t826;
    let t2391 = F::new(3.0) / F::new(10.0) * t313 * (F::new(10.0) / F::new(9.0) * t2359 + F::new(5.0) / F::new(3.0) * t2363 + F::new(10.0) / F::new(9.0) * t2369 - F::new(5.0) / F::new(3.0) * t2373) + t1288;
    (t2381, t2391)
}
