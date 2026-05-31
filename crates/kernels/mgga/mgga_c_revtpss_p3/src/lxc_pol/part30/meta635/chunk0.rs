//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2203/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2203<F: Float>(t13435: F, t7741: F, t2322: F, t28042: F, t13440: F, t5523: F, t25191: F, t7898: F, t1937: F, t49686: F, t75667: F, t13426: F, t6993: F) -> (F, F, F, F, F, F, F, F) {
    let t101534 = F::cast_from(4.0_f64) * t13435 * t7741;
    let t101536 = F::cast_from(4.0_f64) * t2322 * t28042;
    let t101538 = F::cast_from(2.0_f64) * t13440 * t7741;
    let t101540 = F::cast_from(4.0_f64) * t5523 * t28042;
    let t101546 = F::cast_from(6.0_f64) * t7898 * t25191;
    let t101548 = F::cast_from(2.0_f64) * t49686 * t1937;
    let t101550 = F::cast_from(4.0_f64) * t75667 * t1937;
    let t101552 = F::cast_from(4.0_f64) * t13426 * t6993;
    (t101534, t101536, t101538, t101540, t101546, t101548, t101550, t101552)
}
