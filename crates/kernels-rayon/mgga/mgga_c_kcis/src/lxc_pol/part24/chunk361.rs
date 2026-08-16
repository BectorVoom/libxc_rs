//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 361/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk361(t2155: f64, t2157: f64, t2144: f64, t2148: f64, t2151: f64) -> (f64, f64) {
    let t2158 = t2155 * t2157;
    let t2161 = -0.69505208333333333333e-3_f64 * t2158 + 0.69644166666666666665e-2_f64 * t2144;
    let t2165 = 0.1875e0_f64 * t2148 - 0.809375e-1_f64 * t2151;
    (t2161, t2165)
}
