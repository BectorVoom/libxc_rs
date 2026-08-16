//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2288/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2288(t1227: f64, t13969: f64, t18593: f64, t15640: f64, t15737: f64, t15503: f64, t19025: f64, t3535: f64, t1202: f64, t19032: f64, t15498: f64, t4993: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t66084 = t1227 * t13969 * t18593;
    let t66092 = t15737 * t15640;
    let t66120 = t15503 * t15640;
    let t66147 = t3535 * t19025;
    let t66150 = t1202 * t19032;
    let t66153 = t15498 * t4993;
    (t66084, t66092, t66120, t66147, t66150, t66153)
}
