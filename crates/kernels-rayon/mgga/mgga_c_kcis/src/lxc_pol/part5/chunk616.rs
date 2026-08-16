//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 616/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk616(t517: f64, t1075: f64, t317: f64, t522: f64, t3106: f64, t323: f64, t526: f64, t3110: f64, t534: f64, t333: f64, t4016: f64, t532: f64, t833: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4034 = t517 * t517;
    let t4035 = 1.0_f64 / t4034;
    let t4047 = 0.8197e-2_f64 * t317 * t1075 * t522;
    let t4050 = 0.21133333333333333333e-2_f64 * t323 * t3106 * t526;
    let t4051 = t3110 * t534;
    let t4053 = 0.16804375e-4_f64 * t333 * t4051;
    let t4054 = 0.23911438650126355246e-1_f64 * t4016;
    let t4055 = t532 * t833;
    (t4034, t4035, t4047, t4050, t4053, t4054, t4055)
}
