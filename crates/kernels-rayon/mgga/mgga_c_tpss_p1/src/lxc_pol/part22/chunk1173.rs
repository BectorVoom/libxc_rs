//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1173/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1173(t2073: f64, t3532: f64, t640: f64, t2100: f64, t3508: f64, t1324: f64, t2084: f64, t7613: f64, t2083: f64, t97: f64, t1989: f64, t633: f64) -> (f64, f64, f64, f64, f64) {
    let t13164 = t2073 * t3532;
    let t13165 = t13164 * t640;
    let t13168 = t3508 * t2100;
    let t13178 = t7613 * t1324 * t2084;
    let t13181 = t97 * t2083;
    let t13182 = t1989 * t633;
    (t13165, t13168, t13178, t13181, t13182)
}
