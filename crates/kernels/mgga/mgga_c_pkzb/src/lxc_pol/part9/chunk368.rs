//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 368/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk368<F: Float>(t1440: F, t27: F, t1419: F, t1426: F, t1432: F, t1437: F, t16: F, t23: F, t434: F, t441: F, t7: F) -> (F, F) {
    let t1441 = t27 * t1440;
    let t1444 = F::cast_from(88.0_f64) / F::cast_from(9.0_f64) * t1419 * t16 - F::cast_from(80.0_f64) / F::cast_from(9.0_f64) * t434 * t441 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t7 * t1426 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t7 * t1432 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t23 * t1437 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t23 * t1441;
    (t1441, t1444)
}
