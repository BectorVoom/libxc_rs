//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1320/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1320<F: Float>(t1662: F, t301: F, t11856: F, t1298: F, t14866: F, t1674: F, t1680: F, t1946: F, t20007: F, t20008: F, t20009: F, t20010: F, t20011: F, t20013: F, t5403: F, t6592: F, t694: F, t922: F) -> F {
    let t24589 = t301 * t1662;
    let t24601 = F::cast_from(12.0_f64) * t1298 * t5403 * t694 + F::cast_from(6.0_f64) * t1674 * t6592 * t922 - F::cast_from(12.0_f64) * t1680 * t24589 * t694 + F::cast_from(12.0_f64) * t14866 * t1946 + t11856 + t20007 - t20008 - t20009 - t20010 + t20011 + t20013;
    t24601
}
