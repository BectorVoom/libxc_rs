//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 999/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk999(t13042: f64, t1912: f64, t23249: f64, t23252: f64, t23254: f64, t23262: f64, t25230: f64, t25233: f64, t25330: f64, t25339: f64, t25343: f64, t25346: f64, t25348: f64, t2597: f64, t2713: f64, t7517: f64, t855: f64, t866: f64) -> f64 {
    let t25351 = -0.16449340668482264365e-1_f64 * t25230 + 2.0_f64 * t855 * t25233 - t855 * t25330 - 0.19190897446562641759e-1_f64 * t23249 + t23252 - 0.41123351671205660912e-2_f64 * t23254 + t23262 + 2.0_f64 * t2597 * t7517 + 2.0_f64 * t2713 * t7517 - 0.16449340668482264365e-1_f64 * t25339 - 0.16449340668482264365e-1_f64 * t25343 + 0.82246703342411321825e-2_f64 * t25346 - t25348 * t866 - t13042 * t1912;
    t25351
}
