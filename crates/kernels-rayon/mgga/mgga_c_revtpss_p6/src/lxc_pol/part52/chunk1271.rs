//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1271/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1271(t34298: f64, t98588: f64, t2014: f64, t28926: f64, t8717: f64, t128557: f64, t128560: f64, t128562: f64, t128572: f64, t128574: f64, t128577: f64, t128867: f64, t26399: f64, t27145: f64, t28658: f64, t28939: f64, t33913: f64, t7359: f64, t7539: f64, t7746: f64, t8568: f64) -> f64 {
    let t128869 = 2.0_f64 * t98588 * t34298;
    let t128871 = t2014 * t28926 * t8717;
    let t128872 = -2.0_f64 * t26399 * t7746 - 2.0_f64 * t27145 * t7359 - 2.0_f64 * t28658 * t7746 + 3.0_f64 * t28939 * t8568 - t33913 * t7539 - t128557 - t128560 - t128562 + t128572 - t128574 + t128577 + t128867 + t128869 - t128871;
    t128872
}
