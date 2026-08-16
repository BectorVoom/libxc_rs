//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1145/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1145(t14781: f64, t11137: f64, t11139: f64, t11141: f64, t11143: f64, t14728: f64, t14809: f64, t14811: f64, t14814: f64, t14816: f64, t14818: f64, t14824: f64) -> (f64, f64) {
    let t14890 = 0.21908444444444444444e0_f64 * t14781;
    let t14911 = -0.1898925e1_f64 * t14809 - 0.9494625e0_f64 * t14811 + 0.3071625e0_f64 * t14814 + 0.15358125e0_f64 * t14816 + 0.36514074074074074074e-1_f64 * t14818 + 0.26574814814814814816e0_f64 * t11137 + 0.66437037037037037038e-1_f64 * t11139 - 0.19931111111111111111e0_f64 * t11141 - 0.99655555555555555557e-1_f64 * t11143 + 0.3071625e0_f64 * t14824 + 0.33218518518518518518e0_f64 * t14728;
    (t14890, t14911)
}
