//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1408/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1408(t3792: f64, t3850: f64, t1337: f64, t550: f64, t1338: f64, t3879: f64, t3773: f64, t68: f64) -> (f64, f64, f64, f64, f64) {
    let t12240 = t3792 * t3850;
    let t12247 = t1337 * t1337;
    let t12248 = 1.0_f64 / t12247;
    let t12250 = t3792 * t550;
    let t12259 = t1338 * t3879;
    let t12267 = t3773 * t68;
    (t12240, t12248, t12250, t12259, t12267)
}
