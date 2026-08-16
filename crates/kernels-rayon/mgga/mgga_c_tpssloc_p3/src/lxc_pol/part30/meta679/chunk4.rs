//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2131/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2131(t22674: f64, t28205: f64, t6897: f64, t12021: f64, t1375: f64, t16460: f64, t20026: f64, t26477: f64, t5354: f64, t6439: f64, t6958: f64, t6992: f64, t7729: f64, t80663: f64, t80671: f64, t90460: f64, t90469: f64, t90471: f64, t90473: f64, t90498: f64, t90501: f64, t96848: f64, t96851: f64, t96854: f64, t96857: f64, t96866: f64, t96868: f64, t96873: f64) -> f64 {
    let t96878 = t6897 * t22674 * t28205;
    let t96885 = -0.24674011002723396548e-1_f64 * t96848 + 0.16449340668482264365e-1_f64 * t96851 + t90460 + 0.9869604401089358619e-1_f64 * t96854 + t90469 + t90471 - t90473 - 0.82246703342411321825e-2_f64 * t96857 + 4.0_f64 * t16460 * t7729 - 6.0_f64 * t1375 * t12021 * t6992 * t6439 - 0.16449340668482264365e-1_f64 * t96866 + 0.19190897446562641759e-1_f64 * t96868 + 0.16449340668482264365e-1_f64 * t96873 + 2.0_f64 * t6958 * t20026 + 0.41123351671205660912e-2_f64 * t96878 - 2.0_f64 * t26477 * t5354 - 0.63969658155208805863e-1_f64 * t80663 - 0.52089578783527170488e-1_f64 * t80671 - 0.23029076935875170111e0_f64 * t90498 - t90501;
    t96885
}
