//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1098/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1098(t148: f64, t779: f64, t179: f64, t299: f64, t655: f64, t1843: f64, t5672: f64, t5592: f64, t739: f64, t2045: f64, t2057: f64, t2099: f64, t2945: f64, t5684: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18107 = t148 * t779;
    let t18110 = t299 * t179 * t18107 * t655;
    let t18114 = t299 * t179 * t5672 * t1843;
    let t18121 = t5592 * t739;
    let t18123 = t2057 * t2045;
    let t18126 = t2945 * t2099 * t5684;
    (t18107, t18110, t18114, t18121, t18123, t18126)
}
