//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 520/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk520(t2187: f64, t2190: f64, t2193: f64, t2197: f64, t2199: f64, t2202: f64, t676: f64, t657: f64) -> (f64, f64, f64) {
    let t2299 = -0.42198333333333333333e0_f64 * t2187 + 0.84396666666666666666e0_f64 * t2190 + 0.39862222222222222223e0_f64 * t2193 + 0.68258333333333333333e-1_f64 * t2197 + 0.13651666666666666667e0_f64 * t2199 + 0.13692777777777777778e0_f64 * t2202;
    let t2300 = t2299 * t676;
    let t2302 = 1.0_f64 * t657 * t2300;
    (t2299, t2300, t2302)
}
