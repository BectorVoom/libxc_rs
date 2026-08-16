//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1847/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1847(t10416: f64, t1936: f64, t13435: f64, t2322: f64, t7002: f64, t13440: f64, t5523: f64, t112: f64, t239: f64, t624: f64, t655: f64, t665: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t25812 = 2.0_f64 * t10416 * t1936;
    let t25814 = 4.0_f64 * t13435 * t1936;
    let t25816 = 4.0_f64 * t2322 * t7002;
    let t25818 = 2.0_f64 * t13440 * t1936;
    let t25820 = 4.0_f64 * t5523 * t7002;
    let t25821 = t239 * t112;
    let t25822 = 11.0_f64 / 9.0_f64 * t25821;
    let t25823 = t624 * t655;
    let t25824 = t25823 * t665;
    (t25812, t25814, t25816, t25818, t25820, t25822, t25823, t25824)
}
