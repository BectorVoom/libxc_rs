//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3142/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3142(t422: f64, t57883: f64, t57904: f64, t1189: f64, t1196: f64, t17150: f64, t3495: f64, t57820: f64, t57822: f64, t57825: f64, t57827: f64, t57829: f64, t57831: f64, t57833: f64, t57835: f64, t57837: f64, t57840: f64, t57842: f64, t57846: f64, t57849: f64, t57851: f64, t57853: f64, t57856: f64, t57860: f64, t57863: f64) -> (f64, f64, f64) {
    let t57907 = 0.621814e-1_f64 * (t57883 + t57904) * t422;
    let t57911 = 0.35089341735807877242e1_f64 * t1196 * t3495 * t17150 * t1189;
    let t57912 = -t57820 - t57822 - t57825 + t57827 - t57829 - t57831 - t57833 + t57835 + t57837 - t57840 + t57842 + t57846 + t57849 + t57851 + t57853 + t57856 + t57860 - t57863 - t57907 + t57911;
    (t57907, t57911, t57912)
}
