//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 906/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk906(t2376: f64, t339: f64, t795: f64, t803: f64, t2383: f64, t2395: f64, t207: f64, t237: f64, t235: f64, t72: f64, t2146: f64, t756: f64) -> (f64, f64, f64, f64, f64) {
    let t8130 = t339 * t795 * t2376;
    let t8131 = t8130 * t803;
    let t8133 = t2383 * t2395;
    let t8160 = 1.0_f64 / t237 / t207;
    let t8162 = t235 * t8160 * t72;
    let t8167 = t756 * t2146;
    (t8130, t8131, t8133, t8162, t8167)
}
