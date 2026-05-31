//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1503/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1503<F: Float>(t116: F, t31292: F, t117: F, t117103: F, t117575: F, t13514: F, t1459: F, t1461: F, t1518: F, t1916: F, t2327: F, t2371: F, t31114: F, t31117: F, t31124: F, t31340: F, t31359: F, t31362: F, t31365: F, t31370: F, t31371: F, t31374: F, t4158: F, t4292: F, t572: F, t5802: F, t670: F, t8289: F, t8295: F, t8362: F, t8383: F, t8386: F) -> F {
    let t117758 = t116 * t31292;
    let t117765 = F::cast_from(3.0_f64) * t117 * t117575 * t572 + F::cast_from(6.0_f64) * t117103 * t1518 * t572 + F::cast_from(12.0_f64) * t117758 * t572 * t670 + F::cast_from(6.0_f64) * t13514 * t572 * t8295 + F::cast_from(6.0_f64) * t2327 * t572 * t8362 + F::cast_from(6.0_f64) * t2371 * t31370 * t572 + F::cast_from(12.0_f64) * t31117 * t4292 * t572 + F::cast_from(12.0_f64) * t1459 * t31359 + F::cast_from(12.0_f64) * t1459 * t31362 + F::cast_from(12.0_f64) * t1459 * t31365 + F::cast_from(12.0_f64) * t1459 * t31371 + F::cast_from(6.0_f64) * t1459 * t31374 + F::cast_from(6.0_f64) * t1461 * t31340 + F::cast_from(6.0_f64) * t1916 * t31114 + F::cast_from(3.0_f64) * t1916 * t31124 + F::cast_from(6.0_f64) * t4158 * t8383 + F::cast_from(3.0_f64) * t4158 * t8386 + F::cast_from(12.0_f64) * t5802 * t8289;
    t117765
}
