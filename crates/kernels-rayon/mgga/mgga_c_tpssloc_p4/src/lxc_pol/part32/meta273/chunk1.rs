//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1242/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1242(t1510: f64, t6657: f64, t235: f64, t7510: f64, t1499: f64, t1909: f64, t226: f64, t6636: f64, t6645: f64, t7522: f64, t7526: f64, t7530: f64, t812: f64) -> (f64, f64, f64) {
    let t7533 = t6657 * t1510;
    let t7535 = t235 * t7510;
    let t7537 = -t6636 - 0.16449340668482264365e-1_f64 * t7522 - t6645 - 0.82246703342411321825e-2_f64 * t7526 + 0.82246703342411321825e-2_f64 * t7530 + t1499 * t1909 - t812 * t7533 + t226 * t7535;
    (t7533, t7535, t7537)
}
