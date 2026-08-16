//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 743/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk743(t464: f64, t7973: f64, t2122: f64, t315: f64, t323: f64, t309: f64, t2132: f64, t2131: f64, t322: f64, t2138: f64, t7911: f64, t2134: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7974 = t7973 * t464;
    let t7976 = t315 * t2122;
    let t7977 = t7976 * t323;
    let t7979 = t2122 * t309;
    let t7980 = t2132 * t7979;
    let t7981 = t2131 * t7980;
    let t7983 = t2122 * t322;
    let t7984 = t2132 * t7983;
    let t7985 = t2138 * t7984;
    let t7987 = t315 * t7911;
    let t7988 = t7987 * t2134;
    (t7974, t7976, t7977, t7979, t7980, t7981, t7984, t7985, t7987, t7988)
}
