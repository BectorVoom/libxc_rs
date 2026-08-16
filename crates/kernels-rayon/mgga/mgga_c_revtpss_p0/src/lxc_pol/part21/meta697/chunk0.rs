//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2519/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2519(t12690: f64, t1284: f64, t1209: f64, t17948: f64, t3552: f64, t3781: f64, t1204: f64, t13147: f64, t13141: f64, t3596: f64, t42859: f64, t460: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t45726 = t12690 * t1284;
    let t45738 = t1209 * t17948;
    let t45764 = t3552 * t3781;
    let t45769 = t1204 * t13147;
    let t45779 = t1204 * t13141;
    let t45785 = t42859 * t3596;
    let t45786 = t460 * t45785;
    (t45726, t45738, t45764, t45769, t45779, t45785, t45786)
}
