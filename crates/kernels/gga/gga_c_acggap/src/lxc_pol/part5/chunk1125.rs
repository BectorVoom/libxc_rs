//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1125/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1125<F: Float>(t1024: F, t1396: F, t1402: F, t1404: F, t153: F, t1713: F, t1734: F, t1828: F, t1835: F, t19510: F, t20092: F, t301: F, t3220: F, t400: F, t402: F, t420: F, t5060: F, t5066: F, t5506: F, t6045: F, t6053: F, t6056: F, t6061: F, t6065: F, t839: F, t917: F, t921: F, t922: F, t923: F, t94: F) -> F {
    let t20122 = F::new(60.0) * t1024 * t1402 * t1734 * t922 - F::new(360.0) * t1402 * t1713 * t3220 * t922 - F::new(24.0) * t1402 * t301 * t420 * t5506 - F::new(48.0) * t1396 * t1404 * t94 - F::new(12.0) * t1402 * t6061 * t839 + F::new(3.0) * t153 * t19510 * t402 - F::new(24.0) * t153 * t20092 * t921 - F::new(12.0) * t1828 * t923 + F::new(3.0) * t1835 * t917 + F::new(6.0) * t400 * t6065 + F::new(120.0) * t5060 * t6053 - F::new(48.0) * t5060 * t6056 + F::new(120.0) * t5066 * t6045;
    t20122
}
