//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1198/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1198(t121468: f64, t121470: f64, t127428: f64, t127434: f64, t127437: f64, t127477: f64, t127508: f64, t1456: f64, t1458: f64, t1914: f64, t2038: f64, t28283: f64, t32378: f64, t34015: f64, t5790: f64, t7319: f64, t7337: f64, t7940: f64, t7956: f64, t8617: f64) -> f64 {
    let t127511 = 2.0_f64 * t7940 * t7337 + 2.0_f64 * t127428 + 2.0_f64 * t7319 * t7956 + 2.0_f64 * t2038 * t28283 + t121468 + t121470 + t127434 + t5790 * t8617 + t1914 * t32378 + t127437 + t1456 * t34015 + t1458 * (t127477 + t127508);
    t127511
}
