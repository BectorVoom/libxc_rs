//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1385/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1385(t34585: f64, t10615: f64, t31158: f64, t10215: f64, t10495: f64, t1339: f64, t1359: f64, t1424: f64, t1430: f64, t1537: f64, t30788: f64, t30791: f64, t30794: f64, t34556: f64, t34558: f64, t34566: f64, t34567: f64, t34573: f64, t34576: f64, t34579: f64, t34581: f64, t34583: f64, t544: f64, t590: f64, t6716: f64, t6717: f64) -> f64 {
    let t34586 = 0.19171462976960374838e0_f64 * t34585;
    let t34587 = t10615 * t31158;
    let t34588 = 0.17875244975925213335e0_f64 * t34587;
    let t34589 = -0.1022478025437886658e1_f64 * t1537 * t1339 * t10215 * t590 + t34556 + 0.23833659967900284446e0_f64 * t34558 * t1430 - 0.79445533226334281486e-1_f64 * t544 * t1359 * t10495 * t1424 + t30788 + t30791 - t30794 - t34566 + 0.13803453343411469884e2_f64 * t6716 * t6717 * t34567 - t34573 + t34576 + t34579 + t34581 - t34583 - t34586 + t34588;
    t34589
}
