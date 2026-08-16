//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 519/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk519(t1318: f64, t2193: f64, t2129: f64, t2133: f64, t2136: f64, t2139: f64, t2142: f64, t2145: f64, t2148: f64, t2150: f64, t2155: f64, t2160: f64, t2165: f64, t2170: f64, t2173: f64, t2175: f64, t2180: f64, t2185: f64, t2190: f64) -> (f64, f64) {
    let t2195 = 4.0_f64 / 15.0_f64 * t1318 * t2193;
    let t2196 = t2129 + t2133 - t2136 + t2139 + t2142 + t2145 + t2148 + t2150 - t2155 - t2160 + t2165 - t2170 + t2173 + t2175 + t2180 - t2185 + t2190 - t2195;
    (t2195, t2196)
}
