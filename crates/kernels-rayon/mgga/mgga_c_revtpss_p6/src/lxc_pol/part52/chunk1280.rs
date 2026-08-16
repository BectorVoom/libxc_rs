//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1280/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1280(t128970: f64, t128974: f64, t128975: f64, t128977: f64, t128979: f64, t128981: f64, t128983: f64, t128986: f64, t128988: f64, t128990: f64, t128992: f64, t128994: f64, t1453: f64, t28927: f64, t34326: f64, t8568: f64) -> f64 {
    let t128997 = t1453 * t34326 + t28927 * t8568 + t128970 - t128974 + t128975 - t128977 - t128979 - t128981 - t128983 - 2.0_f64 * t128986 - 2.0_f64 * t128988 - 2.0_f64 * t128990 - 2.0_f64 * t128992 - 2.0_f64 * t128994;
    t128997
}
