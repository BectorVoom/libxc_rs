//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 937/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk937(t2902: f64, t673: f64, t2899: f64, t2839: f64, t57: f64, t262: f64, t390: f64, t5543: f64, t1016: f64, t2193: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9194 = t673 * t2902;
    let t9196 = t673 * t2899;
    let t9198 = t2839 * t57;
    let t9199 = 1.0_f64 / t9198;
    let t9213 = t262 * t5543 * t390;
    let t9214 = 0.93932222222222222223e0_f64 * t9213;
    let t9221 = t2193 * t1016;
    (t9194, t9196, t9199, t9213, t9214, t9221)
}
