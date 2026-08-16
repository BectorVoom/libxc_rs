//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1354/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1354(t24823: f64, t85853: f64, t10913: f64, t11639: f64, t11868: f64, t11871: f64, t11877: f64, t11896: f64, t2121: f64, t2147: f64, t24589: f64, t24777: f64, t24788: f64, t24812: f64, t24816: f64, t24822: f64, t27549: f64, t27550: f64, t27551: f64, t27561: f64, t3610: f64, t462: f64, t7373: f64, t7375: f64, t7376: f64, t7386: f64, t7387: f64, t85859: f64, t85863: f64) -> f64 {
    let t85883 = t85853 * t24823;
    let t85895 = 3.0_f64 * t11877 * t7387 - 0.49348022005446793095e-1_f64 * t24812 * t85859 * t24816 + 0.24674011002723396548e-1_f64 * t24812 * t85863 * t24822 + 0.82246703342411321825e-2_f64 * t2121 * t462 * t2147 * t11868 + 0.24674011002723396548e-1_f64 * t7373 * t7375 * t11896 * t7376 - 0.16449340668482264365e-1_f64 * t24589 * t27550 * t27561 * t10913 + 0.82246703342411321825e-2_f64 * t7373 * t7375 * t11639 * t7376 - 0.82246703342411321826e-2_f64 * t85883 + 6.0_f64 * t3610 * t7386 * t11871 + 0.10966227112321509577e-1_f64 * t27549 * t27550 * t27551 * t10913 - 0.10966227112321509577e-1_f64 * t27549 * t24788 * t24777;
    t85895
}
