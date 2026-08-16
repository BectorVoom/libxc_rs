//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1062/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1062(t1314: f64, t2559: f64, t1317: f64, t535: f64, t795: f64, t9580: f64, t3749: f64, t9577: f64, t3726: f64, t3745: f64, t2566: f64, t3741: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12189 = t2559 * t1314;
    let t12190 = t12189 * t1317;
    let t12194 = 0.16435185185185185185e-1_f64 * t9580 * t535 * t795;
    let t12196 = 0.99999999999999999997e-2_f64 * t9577 * t3749;
    let t12197 = t3726 * t3745;
    let t12199 = t2566 * t1314;
    let t12200 = t12199 * t3741;
    (t12189, t12190, t12194, t12196, t12197, t12200)
}
