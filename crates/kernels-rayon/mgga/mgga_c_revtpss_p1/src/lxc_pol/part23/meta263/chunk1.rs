//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1468/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1468(t10022: f64, t786: f64, t2435: f64, t4093: f64, t4083: f64, t9303: f64, t2777: f64, t4092: f64, t2439: f64, t1419: f64, t3999: f64, t123: f64, t212: f64, t2434: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10023 = t786 * t10022;
    let t10032 = t2435 * t4093;
    let t10035 = 0.26019841438354088051e-2_f64 * t9303 * t4083;
    let t10043 = t2777 * t4092;
    let t10044 = t2439 * t10043;
    let t10049 = t3999 * t1419;
    let t10069 = t123 * t2434 * t212;
    (t10023, t10032, t10035, t10043, t10044, t10049, t10069)
}
