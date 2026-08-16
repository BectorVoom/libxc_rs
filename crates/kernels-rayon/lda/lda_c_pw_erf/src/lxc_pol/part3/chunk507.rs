//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 507/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk507(t2120: f64, t496: f64, t514: f64, t834: f64, t211: f64, t1521: f64, t1531: f64, t2044: f64, t2046: f64, t2069: f64, t2074: f64, t2078: f64, t2080: f64, t2102: f64, t2106: f64, t2108: f64, t2110: f64, t2112: f64, t2116: f64, t2118: f64) -> (f64, f64, f64, f64) {
    let t2122 = 4.0_f64 / 15.0_f64 * t2120 * t496;
    let t2123 = t514 * t834;
    let t2124 = t211 * t2123;
    let t2125 = 4.0_f64 / 45.0_f64 * t2124;
    let t2126 = -t1521 + t2044 - t1531 - t2046 - t2069 - t2074 + t2078 - t2080 - t2102 + t2106 + t2108 - t2110 - t2112 + t2116 + t2118 + t2122 - t2125;
    (t2122, t2123, t2125, t2126)
}
