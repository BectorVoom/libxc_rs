//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2184/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2184(t22279: f64, t28167: f64, t8996: f64, t29506: f64, t7313: f64, t105850: f64, t105859: f64, t105863: f64, t105866: f64, t105889: f64, t105894: f64, t105897: f64, t107881: f64, t108062: f64, t118: f64, t1310: f64, t13426: f64, t18220: f64, t18227: f64, t18232: f64, t18245: f64, t1932: f64, t2007: f64, t21658: f64, t29573: f64, t508: f64, t5884: f64, t671: f64, t6765: f64, t6983: f64, t6985: f64, t7007: f64, t7221: f64, t7746: f64) -> f64 {
    let t108067 = 12.0_f64 * t28167 * t8996 * t22279;
    let t108068 = t29506 * t7313;
    let t108071 = -2.0_f64 * t18220 * t2007 - 2.0_f64 * t5884 * t7221 - 2.0_f64 * t105850 * t508 - 2.0_f64 * t29573 * t1310 - t6983 * t6765 - t1932 * t21658 - t105859 - 2.0_f64 * t6985 * t18232 - t105863 - 2.0_f64 * t18245 * t7007 - 2.0_f64 * t105866 * t671 - t105889 - 4.0_f64 * t13426 * t7746 + t105894 + t105897 - t118 * (t107881 + t108062) + t108067 + t108068 - 4.0_f64 * t18227 * t7746;
    t108071
}
