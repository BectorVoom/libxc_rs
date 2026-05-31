//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2052/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2052<F: Float>(t670: F, t7983: F, t101705: F, t1459: F, t1461: F, t1518: F, t1916: F, t1918: F, t2113: F, t21881: F, t22556: F, t22568: F, t26733: F, t28956: F, t28974: F, t28978: F, t28986: F, t30637: F, t30651: F, t30660: F, t4292: F, t572: F, t5795: F, t5805: F, t5883: F, t5920: F, t6941: F, t6948: F, t7373: F, t7547: F, t7553: F, t7557: F, t8118: F, t8127: F) -> F {
    let t111371 = t670 * t7983;
    let t111390 = F::cast_from(3.0_f64) * t2113 * t22568 + F::cast_from(6.0_f64) * t8118 * t5805 + F::cast_from(6.0_f64) * t572 * t5883 * t7373 + F::cast_from(3.0_f64) * t30637 * t1461 + F::cast_from(3.0_f64) * t7547 * t6948 + F::cast_from(3.0_f64) * t6941 * t7557 + F::cast_from(12.0_f64) * t572 * t101705 * t1518 + F::cast_from(12.0_f64) * t572 * t28986 * t4292 + F::cast_from(6.0_f64) * t28956 * t1918 + F::cast_from(3.0_f64) * t1459 * t30660 + F::cast_from(6.0_f64) * t2113 * t22556 + F::cast_from(12.0_f64) * t572 * t111371 * t1518 + F::cast_from(12.0_f64) * t1916 * t28978 + F::cast_from(6.0_f64) * t572 * t28974 * t5920 + F::cast_from(6.0_f64) * t572 * t26733 * t5920 + F::cast_from(6.0_f64) * t572 * t7553 * t21881 + F::cast_from(6.0_f64) * t1459 * t30651 + F::cast_from(6.0_f64) * t5795 * t8127;
    t111390
}
