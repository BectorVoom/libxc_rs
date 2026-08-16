//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2025/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2025(t22773: f64, t22779: f64, t22865: f64, t6604: f64, t6937: f64, t22776: f64, t22811: f64, t61: f64, t133: f64, t1995: f64, t6933: f64, t22803: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t80922 = t22779 * t22773;
    let t80939 = t22865 * t6604;
    let t80940 = t80939 * t6937;
    let t80943 = t22779 * t22776;
    let t80953 = 1.0_f64 / t61 / t22811;
    let t80956 = t80953 * t1995 * t133 * t6933;
    let t80957 = 0.69792532988666768264e-2_f64 * t80956;
    let t80958 = t22803 * t6604;
    (t80922, t80939, t80940, t80943, t80953, t80957, t80958)
}
