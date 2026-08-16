//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1944/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1944<F: Float>(t25: F, t5664: F, t1408: F, t1530: F, t5660: F, t1877: F, t1915: F, t22959: F, t23295: F, t2522: F, t25358: F, t28242: F, t28249: F, t28252: F, t28256: F, t28448: F, t4314: F, t5397: F, t6670: F, t7475: F, t7541: F, t7545: F) -> (F, F, F, F) {
    let t28456 = t25 * t5664;
    let t28459 = t1408 * t1530;
    let t28462 = t25 * t5660;
    let t28469 = F::cast_from(3.0_f64) * t4314 * t28242 + F::cast_from(3.0_f64) * t2522 * t7541 * t7475 - F::cast_from(3.0_f64) * t22959 * t28249 + F::cast_from(3.0_f64) * t2522 * t1915 * t28252 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t1915 * t28256 + t1877 * t28448 * t25 / F::cast_from(2.0_f64) - t1877 * t25358 * t7545 + t1877 * t7541 * t1408 + t1877 * t23295 * t28456 - t1877 * t6670 * t28459 - t1877 * t6670 * t28462 / F::cast_from(2.0_f64) + t1877 * t1915 * t5397 / F::cast_from(2.0_f64);
    (t28456, t28459, t28462, t28469)
}
