//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1095/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1095(t5708: f64, t5713: f64, t2011: f64, t5939: f64, t757: f64, t2026: f64, t2032: f64, t2038: f64, t2040: f64, t1478: f64, t301: f64, t154: f64, t276: f64, t655: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18028 = t5713 * t5708;
    let t18033 = t757 * t5939 * t2011;
    let t18036 = t2026 * t5939 * t2032;
    let t18039 = t2038 * t5939 * t2040;
    let t18060 = t1478 * t301;
    let t18063 = t276 * t154 * t18060 * t655;
    (t18028, t18033, t18036, t18039, t18060, t18063)
}
