//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1406/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1406(t106982: f64, t106986: f64, t106991: f64, t107024: f64, t107048: f64, t107220: f64, t107230: f64, t107238: f64, t107270: f64, t107466: f64, t107486: f64, t12021: f64, t1375: f64, t1390: f64, t1983: f64, t20609: f64, t26477: f64, t28111: f64, t28224: f64, t3887: f64, t5215: f64, t533: f64, t6439: f64, t6460: f64, t6461: f64, t6958: f64, t7749: f64, t90503: f64, t90521: f64, t96848: f64, t96868: f64, t96878: f64, t96893: f64, t97571: f64, t97573: f64, t97599: f64) -> f64 {
    let t107492 = t1983 * t533 * (-3.0_f64 * t26477 * t6461 - 6.0_f64 * t6958 * t20609 - 18.0_f64 * t5215 * t28224 + 6.0_f64 * t5215 * t28111 - 0.38381794893125283518e0_f64 * t90521 + 0.19190897446562641759e0_f64 * t90503 + t107220 + t107270 - 0.74022033008170189643e-1_f64 * t96848 - 0.24674011002723396548e-1_f64 * t106986 + 0.57572692339687925277e-1_f64 * t96868 + 0.12337005501361698274e-1_f64 * t96878 + t107048 + t107466 - 0.49348022005446793095e-1_f64 * t107230 - 0.14804406601634037928e0_f64 * t106991 - 0.12337005501361698274e-1_f64 * t97599 + 0.24674011002723396548e-1_f64 * t96893 + 0.49348022005446793095e-1_f64 * t106982 + t107486 - 0.49348022005446793095e-1_f64 * t107238 + t107024 - 18.0_f64 * t1375 * t12021 * t7749 * t6439 + 6.0_f64 * t1375 * t3887 * t7749 * t6460 - 0.24674011002723396548e-1_f64 * t97571 + 0.11514538467937585055e0_f64 * t97573) * t1390;
    t107492
}
