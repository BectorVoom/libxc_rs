//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 318/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk318(t1015: f64, t1038: f64, t141: f64, t1017: f64, t1028: f64, t1030: f64, t1033: f64, t1037: f64) -> (f64, f64, f64) {
    let t1039 = t1038 * t1015;
    let t1040 = t141 * t1039;
    let t1042 = 0.1898925e1_f64 * t1028 - t1030 + 0.29896666666666666667e0_f64 * t1017 + 0.3071625e0_f64 * t1033 - t1037 + 0.82156666666666666667e-1_f64 * t1040;
    (t1039, t1040, t1042)
}
