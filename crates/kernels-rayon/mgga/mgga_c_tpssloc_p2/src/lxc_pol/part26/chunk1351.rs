//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1351/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1351(t3590: f64, t7284: f64, t11545: f64, t461: f64, t491: f64, t24574: f64, t24630: f64, t24605: f64, t85639: f64, t24888: f64, t10913: f64, t11148: f64, t11172: f64, t11606: f64, t11608: f64, t1238: f64, t24563: f64, t24567: f64, t24589: f64, t24590: f64, t24601: f64, t24633: f64, t24883: f64, t24887: f64, t24897: f64, t27444: f64, t3593: f64, t3599: f64, t7283: f64, t7285: f64, t7286: f64, t7287: f64, t7351: f64, t7391: f64) -> (f64, f64) {
    let t85750 = t7284 * t3590;
    let t85754 = t11545 * t461;
    let t85755 = t85754 * t491;
    let t85766 = t24574 * t24630;
    let t85787 = t85639 * t24605;
    let t85789 = t24574 * t24888;
    let t85791 = -0.82246703342411321826e-2_f64 * t7283 * t85750 * t7287 - 0.8529287754027840782e-2_f64 * t7283 * t85755 * t7286 * t11148 - 18.0_f64 * t1238 * t11606 * t7391 * t3599 - 6.0_f64 * t7351 * t11608 - 0.16449340668482264365e-1_f64 * t85766 - 0.24674011002723396548e-1_f64 * t7283 * t24567 * t24563 - 18.0_f64 * t3593 * t24897 - 0.27415567780803773942e-2_f64 * t7283 * t7285 * t7286 * t11172 - 0.16449340668482264365e-1_f64 * t7283 * t24633 * t24887 + 0.82246703342411321826e-2_f64 * t24589 * t24590 * t24883 - 0.16449340668482264365e-1_f64 * t24589 * t24601 * t27444 * t10913 + 0.54831135561607547883e-2_f64 * t85787 - 0.54831135561607547883e-2_f64 * t85789;
    (t85754, t85791)
}
