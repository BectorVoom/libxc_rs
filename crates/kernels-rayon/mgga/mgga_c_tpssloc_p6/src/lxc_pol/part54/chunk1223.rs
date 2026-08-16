//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1223/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1223(t1510: f64, t31394: f64, t31353: f64, t31355: f64, t31359: f64, t32835: f64, t32838: f64, t32841: f64, t32845: f64, t32847: f64, t235: f64, t1499: f64, t226: f64, t30675: f64, t30683: f64, t31375: f64, t31383: f64, t32821: f64, t32825: f64, t32829: f64, t33377: f64, t33381: f64, t33385: f64, t812: f64, t8560: f64) -> (f64, f64, f64, f64) {
    let t33388 = t31394 * t1510;
    let t33395 = -t31353 - 0.96894614625936938046e-2_f64 * t32835 - t31355 - 0.16149102437656156341e-2_f64 * t32838 + t32841 / 768.0_f64 - t32845 / 768.0_f64 - t31359 - t32847 / 192.0_f64;
    let t33396 = t235 * t33395;
    let t33398 = -t30675 - t32821 - t30683 - t32825 + t32829 - t31375 - 0.16449340668482264365e-1_f64 * t33377 - t31383 - 0.82246703342411321825e-2_f64 * t33381 + 0.82246703342411321825e-2_f64 * t33385 + t1499 * t8560 - t812 * t33388 + t226 * t33396;
    (t33388, t33395, t33396, t33398)
}
