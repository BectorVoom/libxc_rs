//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1615/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1615<F: Float>(t14894: F, t18426: F, t2477: F, t40462: F, t40737: F, t40759: F, t40771: F, t4364: F, t61981: F, t76242: F, t76672: F, t76677: F, t76689: F, t76701: F, t76703: F, t76720: F, t76738: F, t76740: F, t76764: F, t828: F, t851: F, t855: F, t87543: F, t87548: F, t87553: F) -> F {
    let t87608 = F::cast_from(0.12862205435420921092e-1_f64) * t851 * t2477 * t828 * t87548 + F::cast_from(0.18007087609589289528e0_f64) * t851 * t40462 * t828 * t87553 - F::cast_from(0.85748036236139473944e-3_f64) * t851 * t855 * t828 * t87543 - F::cast_from(0.77173232612525526552e-2_f64) * t14894 * t4364 * t18426 * t76242 + F::cast_from(0.6098400337114239387e-3_f64) * t76672 + t40737 - F::new(7.0) / F::new(4.0) * t76677 + F::cast_from(0.60984003371142393869e-3_f64) * t76689 - F::cast_from(0.30492001685571196936e-2_f64) * t76701 - F::cast_from(0.24009450146119052704e-1_f64) * t76703 + F::cast_from(0.60984003371142393869e-3_f64) * t76720 - F::cast_from(0.12196800674228478774e-2_f64) * t76738 + F::cast_from(0.24009450146119052704e0_f64) * t76740 - F::cast_from(0.27107389498472794074e-4_f64) * t61981 - t40759 + t40771 + F::cast_from(0.85748036236139473944e-4_f64) * t76764;
    t87608
}
