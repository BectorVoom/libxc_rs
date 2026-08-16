//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1026/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1026(t11040: f64, t847: f64, t11026: f64, t861: f64, t141: f64, t11013: f64, t2515: f64, t3800: f64, t673: f64, t3797: f64, t10990: f64, t10992: f64, t10994: f64, t8647: f64, t8661: f64, t8665: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11041 = t847 * t11040;
    let t11043 = t861 * t11026;
    let t11044 = t141 * t11043;
    let t11046 = t2515 * t11013;
    let t11047 = t141 * t11046;
    let t11049 = t673 * t3800;
    let t11050 = 0.21908444444444444444e0_f64 * t11049;
    let t11051 = t673 * t3797;
    let t11053 = t10990 - 0.82156666666666666667e-1_f64 * t10992 - 0.91285185185185185185e-1_f64 * t10994 - 0.10954222222222222222e0_f64 * t8647 - t8661 - t8665 + 0.1898925e1_f64 * t11041 - 0.49293999999999999999e0_f64 * t11044 + 0.16431333333333333333e0_f64 * t11047 - t11050 + 0.36514074074074074074e-1_f64 * t11051;
    (t11041, t11044, t11047, t11049, t11051, t11053)
}
