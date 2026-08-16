//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 762/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk762(t2133: f64, t6303: f64, t120: f64, t122: f64, t135: f64, t273: f64, t57: f64, t2096: f64, t784: f64, t23: f64, t271: f64, t6077: f64) -> (f64, f64, f64, f64) {
    let t6304 = t2133 * t6303;
    let t6310 = 0.92480845007273388189e0_f64 * t120 * t122 * t273 * t57 * t135;
    let t6311 = t2096 * t784;
    let t6314 = 1.0_f64 / t23 / t6077 / t271;
    (t6304, t6310, t6311, t6314)
}
