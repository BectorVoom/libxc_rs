//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2334/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2334<F: Float>(t20949: F, t2697: F, t20882: F, t9638: F, t13258: F, t20988: F, t13251: F, t16853: F, t16946: F, t16949: F, t16976: F, t17013: F, t2643: F, t2645: F, t41467: F, t4172: F, t4248: F, t4257: F, t46550: F, t46628: F, t5591: F, t58461: F, t58472: F, t58474: F, t58495: F, t9642: F) -> F {
    let t67675 = t2697 * t20949;
    let t67690 = t9638 * t20882;
    let t67692 = t13258 * t20988;
    let t67696 = -F::cast_from(15.0_f64) / F::cast_from(128.0_f64) * t4172 * t16853 + F::cast_from(5.0_f64) / F::cast_from(128.0_f64) * t4172 * t16946 + F::cast_from(5.0_f64) / F::cast_from(256.0_f64) * t16976 * t4257 - F::cast_from(35.0_f64) / F::cast_from(384.0_f64) * t67675 + t46550 - t13251 * t17013 / F::cast_from(1024.0_f64) + F::cast_from(35.0_f64) / F::cast_from(384.0_f64) * t58461 - F::cast_from(15.0_f64) / F::cast_from(128.0_f64) * t46628 * t41467 * t4248 * t16949 + t9642 * t20882 / F::cast_from(256.0_f64) + t2643 * t2645 * t58495 * t5591 / F::cast_from(256.0_f64) - F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t67690 - F::cast_from(7.0_f64) / F::cast_from(768.0_f64) * t67692 - F::cast_from(7.0_f64) / F::cast_from(768.0_f64) * t58472 - F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t58474;
    t67696
}
