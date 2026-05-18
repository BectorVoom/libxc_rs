//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 900/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk900<F: Float>(t1010: F, t1271: F, t1276: F, t2378: F, t2381: F, t2391: F, t2928: F, t2938: F, t321: F, t6654: F, t6661: F, t819: F, t826: F, t8355: F, t8358: F, t9638: F, t9640: F, t9650: F, t9653: F, t9657: F, t9673: F) -> F {
    let t9675 = -F::new(2.0) * t8355 * t1010 - t1271 * t2938 + F::new(4.0) * t1276 * t9653 + F::new(2.0) * t1276 * t9657 - F::new(2.0) * t2378 * t2391 + F::new(4.0) * t8358 * t2381 + F::new(2.0) * t6654 * t2928 + t9638 * t321 - F::new(6.0) * t6661 * t9650 - t819 * t9673 - t9640 * t826;
    t9675
}
