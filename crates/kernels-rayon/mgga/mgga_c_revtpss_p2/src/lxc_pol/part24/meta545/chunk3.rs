//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1615/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1615(t14894: f64, t18426: f64, t2477: f64, t40462: f64, t40737: f64, t40759: f64, t40771: f64, t4364: f64, t61981: f64, t76242: f64, t76672: f64, t76677: f64, t76689: f64, t76701: f64, t76703: f64, t76720: f64, t76738: f64, t76740: f64, t76764: f64, t828: f64, t851: f64, t855: f64, t87543: f64, t87548: f64, t87553: f64) -> f64 {
    let t87608 = 0.12862205435420921092e-1_f64 * t851 * t2477 * t828 * t87548 + 0.18007087609589289528e0_f64 * t851 * t40462 * t828 * t87553 - 0.85748036236139473944e-3_f64 * t851 * t855 * t828 * t87543 - 0.77173232612525526552e-2_f64 * t14894 * t4364 * t18426 * t76242 + 0.6098400337114239387e-3_f64 * t76672 + t40737 - 7.0_f64 / 4.0_f64 * t76677 + 0.60984003371142393869e-3_f64 * t76689 - 0.30492001685571196936e-2_f64 * t76701 - 0.24009450146119052704e-1_f64 * t76703 + 0.60984003371142393869e-3_f64 * t76720 - 0.12196800674228478774e-2_f64 * t76738 + 0.24009450146119052704e0_f64 * t76740 - 0.27107389498472794074e-4_f64 * t61981 - t40759 + t40771 + 0.85748036236139473944e-4_f64 * t76764;
    t87608
}
