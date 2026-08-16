//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 929/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk929(t2132: f64, t2133: f64, t7896: f64, t879: f64, t3915: f64, t7948: f64, t2147: f64, t463: f64, t7885: f64, t7886: f64, t1960: f64, t3912: f64) -> (f64, f64, f64, f64) {
    let t32052 = 0.52041769129231196772e1_f64 * t7896 * t2132 * t2133 * t879;
    let t32054 = 0.39512695097613069591e1_f64 * t7948 * t3915;
    let t32061 = t7885 * t2147 * t7886 * t463;
    let t32073 = 0.65854491829355115987e0_f64 * t1960 * t3912;
    (t32052, t32054, t32061, t32073)
}
