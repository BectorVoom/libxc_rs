//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 722/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk722(t1636: f64, t2268: f64, t7946: f64, t7950: f64, t7954: f64, t7956: f64, t7958: f64, t7960: f64) -> (f64, f64) {
    let t8001 = t2268 * t1636;
    let t8010 = 0.9375e-1_f64 * t7946 - 0.9375e-1_f64 * t7950 + 0.625e-1_f64 * t7954 - 0.20234375e-1_f64 * t7956 + 0.20234375e-1_f64 * t7958 - 0.26979166666666666667e-1_f64 * t7960;
    (t8001, t8010)
}
