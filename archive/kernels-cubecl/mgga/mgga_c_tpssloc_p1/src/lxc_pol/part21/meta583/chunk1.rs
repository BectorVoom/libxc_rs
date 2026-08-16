//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2312/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2312<F: Float>(t1365: F, t6347: F, t1307: F, t1347: F, t19631: F, t1345: F, t1348: F, t1819: F, t1821: F, t19702: F, t19708: F, t19716: F, t19719: F, t5272: F, t5278: F, t5280: F, t5283: F, t546: F, t548: F, t6404: F, t6408: F, t6411: F) -> (F, F, F, F) {
    let t19724 = t1365 * t6347;
    let t19725 = t19724 * t1307;
    let t19728 = t1347 * t19631;
    let t19731 = -F::cast_from(12.0_f64) * t1345 * t6408 + F::cast_from(3.0_f64) * t1345 * t6411 + F::cast_from(3.0_f64) * t1348 * t6404 + F::cast_from(6.0_f64) * t1819 * t5283 + F::cast_from(6.0_f64) * t1821 * t5272 - t19702 * t548 - F::cast_from(24.0_f64) * t19708 * t5280 + F::cast_from(60.0_f64) * t19716 * t5278 - F::cast_from(24.0_f64) * t19719 * t5278 - F::cast_from(12.0_f64) * t19725 * t5278 + F::cast_from(3.0_f64) * t19728 * t546;
    (t19724, t19725, t19728, t19731)
}
