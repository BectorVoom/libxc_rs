//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1307/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1307(t13796: f64, t14423: f64, t3166: f64, t3989: f64, t56296: f64, t875: f64, t14397: f64, t3083: f64, t1113: f64, t13776: f64, t3747: f64, t3975: f64, t810: f64) -> (f64, f64, f64, f64) {
    let t56697 = t3989 * t13796 * t14423 * t3166;
    let t56701 = t3989 * t13796 * t56296 * t875;
    let t56703 = t3083 * t14397;
    let t56708 = t13776 * t3975 * t1113 * t3747 * t810;
    (t56697, t56701, t56703, t56708)
}
