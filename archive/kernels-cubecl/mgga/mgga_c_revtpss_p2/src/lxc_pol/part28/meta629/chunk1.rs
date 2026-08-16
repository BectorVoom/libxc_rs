//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2266/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2266<F: Float>(t10416: F, t7741: F, t13435: F, t2322: F, t28042: F, t13440: F, t5523: F, t101407: F, t101517: F, t101519: F, t101521: F, t101524: F, t101526: F, t101528: F, t101530: F, t97593: F) -> F {
    let t101532 = F::cast_from(2.0_f64) * t10416 * t7741;
    let t101534 = F::cast_from(4.0_f64) * t13435 * t7741;
    let t101536 = F::cast_from(4.0_f64) * t2322 * t28042;
    let t101538 = F::cast_from(2.0_f64) * t13440 * t7741;
    let t101540 = F::cast_from(4.0_f64) * t5523 * t28042;
    let t101542 = t101517 + t101519 + t101521 + t101524 + t101526 + t101528 + t101530 + t101532 + t101534 + t101536 + t101538 + t101540 + F::cast_from(2.0_f64) * t97593 + t101407;
    t101542
}
