//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 632/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk632<F: Float>(t1734: F, t420: F, t301: F, t402: F, t5506: F, t1396: F, t1402: F, t1404: F, t1407: F, t153: F, t155: F, t1828: F, t1832: F, t1835: F, t400: F, t403: F, t519: F, t521: F, t6039: F, t6045: F, t6053: F, t6056: F) -> F {
    let t6061 = t420 * t1734;
    let t6062 = t6061 * t301;
    let t6065 = t402 * t5506;
    let t6068 = F::cast_from(6.0_f64) * t1396 * t521 + F::cast_from(60.0_f64) * t1402 * t6053 - F::cast_from(24.0_f64) * t1402 * t6056 - F::cast_from(12.0_f64) * t1402 * t6062 - F::cast_from(24.0_f64) * t1404 * t6045 + F::cast_from(6.0_f64) * t1407 * t519 + F::cast_from(3.0_f64) * t153 * t6065 - t155 * t6039 + F::cast_from(3.0_f64) * t1828 * t403 - F::cast_from(12.0_f64) * t1832 * t400 + F::cast_from(3.0_f64) * t1835 * t400;
    t6068
}
