//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1425/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1425(t3886: f64, t7936: f64, t1385: f64, t1992: f64, t22635: f64, t31559: f64, t90566: f64, t33246: f64, t6883: f64, t115339: f64, t115341: f64, t120218: f64, t120221: f64, t120226: f64, t120229: f64, t2092: f64, t26477: f64, t27115: f64, t31642: f64, t5215: f64, t6958: f64, t7214: f64, t91491: f64) -> f64 {
    let t122142 = t3886 * t7936;
    let t122145 = t1992 * t22635 * t122142 * t1385;
    let t122150 = t1992 * t90566 * t31559;
    let t122152 = t6883 * t33246;
    let t122155 = -t6958 * t27115 + 0.38381794893125283518e-1_f64 * t115339 + 0.19190897446562641759e-1_f64 * t115341 + 0.16449340668482264365e-1_f64 * t122145 - t5215 * t31642 - t91491 * t2092 - t120218 - t120221 + 0.16449340668482264365e-1_f64 * t122150 - 0.19190897446562641759e-1_f64 * t122152 + t120226 - t26477 * t7214 + t120229;
    t122155
}
