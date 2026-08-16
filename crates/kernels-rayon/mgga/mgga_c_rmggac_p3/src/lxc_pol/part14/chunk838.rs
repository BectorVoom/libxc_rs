//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 838/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk838(t1550: f64, t2060: f64, t27146: f64, t1356: f64, t1364: f64, t2024: f64, t27124: f64, t27177: f64, t30900: f64, t34688: f64, t36280: f64, t38643: f64, t38645: f64, t38648: f64, t38653: f64, t38658: f64, t38663: f64, t38676: f64, t38678: f64, t38680: f64, t4985: f64, t739: f64, t7533: f64, t7567: f64, t8377: f64) -> f64 {
    let t38685 = t1550 * t2060 * t27146;
    let t38693 = -0.59590439850616975157e-4_f64 * t38643 + 0.59590439850616975157e-4_f64 * t38645 + t38648 - t34688 + 0.25538759935978703638e-4_f64 * t38653 - 0.25538759935978703638e-4_f64 * t38658 - 0.85129199786595678796e-5_f64 * t38663 + 0.23948483403727617128e0_f64 * t739 * t7567 * t8377 + 0.23948483403727617128e0_f64 * t739 * t2024 * t27146 + 0.11974241701863808564e0_f64 * t739 * t2024 * t27124 - t38676 + 0.2993560425465952141e-1_f64 * t38678 + 0.5987120850931904282e-1_f64 * t38680 - 0.23948483403727617128e0_f64 * t4985 * t7533 + 0.5987120850931904282e-1_f64 * t38685 + 0.47896966807455234256e0_f64 * t1364 * t2024 * t27177 + 0.47896966807455234256e0_f64 * t1356 * t36280 * t30900;
    t38693
}
