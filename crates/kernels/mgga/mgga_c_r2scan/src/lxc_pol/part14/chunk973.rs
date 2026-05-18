//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 973/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk973<F: Float>(t10982: F, t10990: F, t10995: F, t10741: F, t10699: F, t10702: F, t10705: F, t10712: F, t10714: F, t10717: F, t10720: F, t10723: F, t10726: F, t10730: F, t10732: F, t10744: F, t10746: F, t10749: F) -> (F, F, F, F, F) {
    let t11378 = F::new(0.86737941314158990616e-4) * t10982;
    let t11379 = F::new(0.29810146462873361016e-2) * t10990;
    let t11380 = F::new(0.60975299583150056624e-3) * t10995;
    let t11393 = F::new(0.31147743054556651237e-1) * t10741;
    let t11397 = F::new(0.25610080155860322884e0) * t10699 - F::new(0.54878743191129263322e-1) * t10702 + F::new(0.87327386630866483588e-2) * t10705 + F::new(0.28565981518604370584e-1) * t10712 - F::new(0.17336443480108537126e0) * t10714 + F::new(0.10975748638225852664e0) * t10717 + F::new(0.17336443480108537126e0) * t10720 + F::new(0.5200933044032561138e0) * t10723 - F::new(0.86682217400542685632e-1) * t10726 + F::new(0.95219938395347901946e-2) * t10730 - F::new(0.95219938395347901946e-2) * t10732 - t11393 + F::new(0.51220160311720645767e0) * t10744 - F::new(0.10975748638225852664e0) * t10746 + F::new(0.32927245914677557992e0) * t10749;
    (t11378, t11379, t11380, t11393, t11397)
}
