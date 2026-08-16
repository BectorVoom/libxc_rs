//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1242/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1242(t2045: f64, t8240: f64, t122806: f64, t122809: f64, t123124: f64, t123129: f64, t129530: f64, t129531: f64, t129533: f64, t129534: f64, t129540: f64, t129552: f64, t129566: f64, t129580: f64, t1456: f64, t1458: f64, t1914: f64, t2038: f64, t29490: f64, t32910: f64, t34490: f64, t5790: f64, t7691: f64, t7700: f64, t7940: f64, t7956: f64, t8776: f64) -> f64 {
    let t129585 = t8240 * t2045;
    let t129589 = t123124 + t129530 + t129531 + t7691 * t7956 + t122809 + t123129 + t129533 + t129534 + t2038 * t29490 + t122806 + t1458 * (t129540 + t129552 + t129566 + t129580) + t7940 * t7700 + t129585 + t1456 * t34490 + t1914 * t32910 + t5790 * t8776;
    t129589
}
