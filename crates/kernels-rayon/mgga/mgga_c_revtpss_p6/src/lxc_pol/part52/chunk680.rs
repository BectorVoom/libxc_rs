//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 680/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk680(t1976: f64, t988: f64, t7145: f64, t1981: f64, t3056: f64, t7143: f64, t999: f64, t1071: f64, t1982: f64, t3268: f64, t359: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7146 = t1976 * t988;
    let t7147 = t7145 * t7146;
    let t7150 = t1981 * t3056;
    let t7151 = t7150 * t7143;
    let t7152 = t1976 * t999;
    let t7153 = t7145 * t7152;
    let t7156 = t1982 * t1071;
    let t7159 = t1982 * t7143;
    let t7160 = t3268 * t359;
    (t7147, t7150, t7151, t7153, t7156, t7159, t7160)
}
