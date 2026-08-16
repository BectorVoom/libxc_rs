//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 272/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk272(t837: f64, t861: f64, t141: f64, t839: f64, t850: f64, t852: f64, t855: f64, t860: f64) -> (f64, f64, f64) {
    let t862 = t861 * t837;
    let t863 = t141 * t862;
    let t865 = 0.1898925e1_f64 * t850 - t852 - 0.29896666666666666667e0_f64 * t839 + 0.3071625e0_f64 * t855 - t860 - 0.82156666666666666667e-1_f64 * t863;
    (t862, t863, t865)
}
