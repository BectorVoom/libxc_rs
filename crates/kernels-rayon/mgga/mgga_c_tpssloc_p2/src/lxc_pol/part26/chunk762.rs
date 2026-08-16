//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 762/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk762(t2132: f64, t52: f64, t2136: f64, t6729: f64, t1184: f64, t460: f64) -> (f64, f64, f64) {
    let t7313 = t2132 * t52;
    let t7315 = 0.10093189023535097714e-3_f64 * t7313 * t2136;
    let t7316 = t2132 * t6729;
    let t7319 = t1184 * t460;
    (t7315, t7316, t7319)
}
