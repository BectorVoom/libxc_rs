//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2219/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2219<F: Float>(t670: F, t7583: F, t101530: F, t101532: F, t101534: F, t101536: F, t101538: F, t101540: F, t104115: F, t104138: F, t13514: F, t1518: F, t2371: F, t27060: F, t29427: F, t29432: F, t4292: F, t7586: F, t96706: F) -> (F, F) {
    let t104416 = t7583 * t670;
    let t104427 = F::cast_from(4.0_f64) * t104115 * t670 + F::cast_from(2.0_f64) * t104138 * t1518 + F::cast_from(4.0_f64) * t104416 * t1518 + F::cast_from(2.0_f64) * t13514 * t7586 + F::cast_from(2.0_f64) * t1518 * t96706 + F::cast_from(2.0_f64) * t2371 * t29427 + F::cast_from(4.0_f64) * t27060 * t4292 + F::cast_from(4.0_f64) * t29432 * t4292 + t101530 + t101532 + t101534 + t101536 + t101538 + t101540;
    (t104416, t104427)
}
