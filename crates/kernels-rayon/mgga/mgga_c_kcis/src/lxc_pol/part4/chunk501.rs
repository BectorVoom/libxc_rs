//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 501/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk501(t12: f64, t20: f64, t2317: f64, t2320: f64, t656: f64, t22: f64, t737: f64) -> (f64, f64, f64, f64) {
    let t2325 = 1.0_f64/f64::sqrt(t12);
    let t2326 = t2325 * t20;
    let t2327 = t2326 * t2317;
    let t2329 = t656 * t2320;
    let t2331 = t22 * t737;
    (t2326, t2327, t2329, t2331)
}
