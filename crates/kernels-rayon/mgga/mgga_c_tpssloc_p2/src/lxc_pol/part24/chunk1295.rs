//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1295/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1295(t12030: f64, t12033: f64, t12444: f64, t1390: f64, t1983: f64, t22670: f64, t22905: f64, t3758: f64, t3889: f64, t533: f64, t6963: f64, t6993: f64, t80643: f64, t80647: f64, t80652: f64, t80656: f64, t80659: f64, t80663: f64, t80665: f64, t80667: f64, t80702: f64, t80740: f64, t81278: f64, t81282: f64, t81284: f64, t81291: f64, t81300: f64, t81305: f64, t81307: f64, t81311: f64, t81348: f64, t81377: f64, t81404: f64) -> f64 {
    let t81410 = t1983 * t533 * (-3.0_f64 * t12030 * t6993 + 6.0_f64 * t22670 * t3889 - 3.0_f64 * t12033 * t6993 - 3.0_f64 * t3758 * t22905 + 12.0_f64 * t12444 * t6963 + t81404 + t81377 + t81348 - 0.24674011002723396547e-1_f64 * t81311 + 0.49348022005446793095e-1_f64 * t81305 - 0.57572692339687925277e-1_f64 * t81307 - 0.14804406601634037928e0_f64 * t81300 + 0.82246703342411321825e-2_f64 * t81291 + 0.49348022005446793095e-1_f64 * t81284 + t81282 + t81278 + t80740 + t80702 + 0.23029076935875170111e0_f64 * t80665 + 0.11514538467937585055e0_f64 * t80667 - 0.19190897446562641759e0_f64 * t80663 - 0.16449340668482264365e-1_f64 * t80656 + 0.24674011002723396548e-1_f64 * t80659 + 0.9869604401089358619e-1_f64 * t80652 + 0.24674011002723396547e-1_f64 * t80647 - 0.49348022005446793095e-1_f64 * t80643) * t1390;
    t81410
}
