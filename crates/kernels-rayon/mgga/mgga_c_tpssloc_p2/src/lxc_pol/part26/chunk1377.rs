//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1377/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1377(t225: f64, t24871: f64, t2122: f64, t7319: f64, t24574: f64, t24597: f64, t1235: f64, t461: f64, t24626: f64, t24617: f64, t11498: f64, t1190: f64, t11925: f64, t1252: f64, t2123: f64, t24567: f64, t24582: f64, t24589: f64, t24590: f64, t24596: f64, t24604: f64, t24616: f64, t24757: f64, t24877: f64, t27549: f64, t3487: f64, t3593: f64, t498: f64, t7283: f64, t7356: f64) -> f64 {
    let t86400 = t24871 * t225;
    let t86403 = t7319 * t2122;
    let t86409 = t24574 * t24597;
    let t86415 = t461 * t1235 * t225;
    let t86424 = t24574 * t24626;
    let t86426 = t24574 * t24617;
    let t86436 = -3.0_f64 * t86400 * t1252 - 0.16449340668482264365e-1_f64 * t24589 * t86403 * t24604 + 12.0_f64 * t3593 * t24582 + 0.36554090374405031922e-2_f64 * t86409 + 0.49348022005446793095e-1_f64 * t7283 * t24567 * t24616 + 0.16449340668482264365e-1_f64 * t24589 * t86415 * t24604 - 0.10966227112321509577e-1_f64 * t27549 * t24590 * t24596 + 6.0_f64 * t11925 * t7356 - 0.82246703342411321826e-2_f64 * t86424 + 0.16449340668482264365e-1_f64 * t86426 + 6.0_f64 * t3487 * t24877 + 3.0_f64 * t1190 * t24757 * t498 - 0.82246703342411321825e-2_f64 * t7283 * t11498 * t2123;
    t86436
}
