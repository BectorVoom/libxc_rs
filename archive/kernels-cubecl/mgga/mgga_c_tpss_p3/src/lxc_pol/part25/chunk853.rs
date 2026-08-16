//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 853/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk853<F: Float>(t1165: F, t1338: F, t1799: F, t3493: F, t5801: F, t6234: F, t6309: F, t6323: F, t5909: F, t6245: F, t5913: F, t5916: F, t6249: F, t6251: F, t6253: F) -> (F, F, F) {
    let t6409 = F::cast_from(2.0_f64) * t1165 * t6323 + F::cast_from(2.0_f64) * t1338 * t5801 + F::cast_from(2.0_f64) * t1799 * t3493 + F::cast_from(2.0_f64) * t1799 * t6234 + t6309;
    let t6413 = t5909 * t6245;
    let t6419 = -t5913 - t6249 / F::cast_from(24.0_f64) - t6251 / F::cast_from(768.0_f64) - t5916 - t6253 / F::cast_from(192.0_f64);
    (t6409, t6413, t6419)
}
