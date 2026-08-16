//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 818/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk818(t1932: f64, t704: f64, t1940: f64, t702: f64, t1971: f64, t723: f64, t1979: f64, t721: f64, t1915: f64, t690: f64, t5831: f64, t703: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5883 = t704 * t1932;
    let t5887 = t1932 * t1940 * t702;
    let t5890 = t723 * t1971;
    let t5893 = t1971 * t1979;
    let t5894 = t5893 * t721;
    let t5897 = t690 * t1915;
    let t5900 = t5831 * t703;
    (t5883, t5887, t5890, t5893, t5894, t5897, t5900)
}
