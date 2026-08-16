//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2335/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2335(t29787: f64, t85639: f64, t1170: f64, t2121: f64, t29726: f64, t103337: f64, t104453: f64, t1244: f64, t1246: f64, t15027: f64, t1716: f64, t19201: f64, t2147: f64, t27454: f64, t27471: f64, t27507: f64, t27511: f64, t27543: f64, t27725: f64, t470: f64, t491: f64, t4928: f64, t493: f64, t5064: f64, t6218: f64, t7283: f64, t7348: f64, t7387: f64, t95768: f64, t95774: f64) -> f64 {
    let t104469 = t85639 * t29787;
    let t104480 = t2121 * t1170 * t29726;
    let t104482 = t470 * t493 * t104453 + 0.97477574331746751793e-2_f64 * t95768 - 0.16449340668482264365e-1_f64 * t7283 * t1716 * t2147 * t491 * t4928 + 2.0_f64 * t5064 * t27471 + t1244 * t7348 * t6218 * t1246 + t95774 - 0.43864908449286038306e-1_f64 * t27507 * t27511 + 0.18277045187202515961e-2_f64 * t104469 - 0.82246703342411321825e-2_f64 * t7283 * t103337 * t27454 + 2.0_f64 * t5064 * t27725 + t19201 * t7387 + 4.0_f64 * t15027 * t27543 + 0.27415567780803773942e-2_f64 * t104480;
    t104482
}
