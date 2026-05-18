//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 908/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk908<F: Float>(t1010: F, t1271: F, t1276: F, t1277: F, t1289: F, t2378: F, t2381: F, t2391: F, t321: F, t6651: F, t6654: F, t6661: F, t819: F, t826: F, t8353: F, t8355: F, t8358: F, t8367: F, t8370: F, t8373: F, t8395: F) -> F {
    let t8397 = -t6651 * t1010 - F::new(2.0) * t1271 * t2391 + F::new(4.0) * t1276 * t8370 + F::new(2.0) * t1276 * t8373 + F::new(2.0) * t8358 * t1277 - t2378 * t1289 + F::new(4.0) * t6654 * t2381 + t8353 * t321 - F::new(6.0) * t6661 * t8367 - t819 * t8395 - F::new(2.0) * t8355 * t826;
    t8397
}
