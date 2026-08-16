//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1371/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1371(t2717: f64, t7841: f64, t1888: f64, t23270: f64, t865: f64, t25038: f64, t31337: f64, t4255: f64, t112676: f64, t114613: f64, t114615: f64, t118500: f64, t118503: f64, t118506: f64, t118518: f64, t118523: f64, t2597: f64, t2713: f64, t2718: f64, t33443: f64, t7106: f64, t7537: f64, t855: f64) -> f64 {
    let t121349 = t2717 * t7841;
    let t121352 = t1888 * t23270 * t121349 * t865;
    let t121362 = t25038 * t23270 * t31337 * t4255;
    let t121364 = 2.0_f64 * t2597 * t33443 + 2.0_f64 * t2713 * t33443 + 0.16449340668482264365e-1_f64 * t121352 - 0.82246703342411321824e-2_f64 * t114613 + t118500 - t112676 + 2.0_f64 * t855 * t2718 * t7106 * t7537 - 0.19190897446562641759e-1_f64 * t114615 - t118503 - 0.49348022005446793095e-1_f64 * t121362 + t118506 - t118518 - t118523;
    t121364
}
