//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 973/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk973(t10982: f64, t10990: f64, t10995: f64, t10741: f64, t10699: f64, t10702: f64, t10705: f64, t10712: f64, t10714: f64, t10717: f64, t10720: f64, t10723: f64, t10726: f64, t10730: f64, t10732: f64, t10744: f64, t10746: f64, t10749: f64) -> (f64, f64, f64, f64, f64) {
    let t11378 = 0.86737941314158990616e-4_f64 * t10982;
    let t11379 = 0.29810146462873361016e-2_f64 * t10990;
    let t11380 = 0.60975299583150056624e-3_f64 * t10995;
    let t11393 = 0.31147743054556651237e-1_f64 * t10741;
    let t11397 = 0.25610080155860322884e0_f64 * t10699 - 0.54878743191129263322e-1_f64 * t10702 + 0.87327386630866483588e-2_f64 * t10705 + 0.28565981518604370584e-1_f64 * t10712 - 0.17336443480108537126e0_f64 * t10714 + 0.10975748638225852664e0_f64 * t10717 + 0.17336443480108537126e0_f64 * t10720 + 0.5200933044032561138e0_f64 * t10723 - 0.86682217400542685632e-1_f64 * t10726 + 0.95219938395347901946e-2_f64 * t10730 - 0.95219938395347901946e-2_f64 * t10732 - t11393 + 0.51220160311720645767e0_f64 * t10744 - 0.10975748638225852664e0_f64 * t10746 + 0.32927245914677557992e0_f64 * t10749;
    (t11378, t11379, t11380, t11393, t11397)
}
