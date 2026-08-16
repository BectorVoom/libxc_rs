//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1227/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1227(t235: f64, t33395: f64, t1499: f64, t226: f64, t30675: f64, t30683: f64, t31375: f64, t31383: f64, t32821: f64, t32825: f64, t32829: f64, t33377: f64, t33381: f64, t33385: f64, t33388: f64, t812: f64, t8560: f64) -> (f64, f64) {
    let t33396 = t235 * t33395;
    let t33398 = -t30675 - t32821 - t30683 - t32825 + t32829 - t31375 - 0.16449340668482264365e-1_f64 * t33377 - t31383 - 0.82246703342411321825e-2_f64 * t33381 + 0.82246703342411321825e-2_f64 * t33385 + t1499 * t8560 - t812 * t33388 + t226 * t33396;
    (t33396, t33398)
}
