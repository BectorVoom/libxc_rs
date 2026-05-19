//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1216/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1216<F: Float>(t37835: F, t37838: F, t37841: F, t37843: F, t37848: F, t37851: F, t38528: F, t38532: F, t39740: F, t39742: F, t39746: F, t39749: F) -> F {
    let t41537 = t38528 + t38532 + F::cast_from(0.11708928647259339623e0_f64) * t37835 + F::cast_from(0.90044238659382329742e0_f64) * t37838 + F::cast_from(0.27013271597814698923e1_f64) * t37841 - F::cast_from(0.17336443480108537126e0_f64) * t39740 - F::cast_from(0.86682217400542685632e-1_f64) * t39742 - F::cast_from(0.5200933044032561138e0_f64) * t39746 + F::cast_from(0.26198215989259945076e-1_f64) * t39749 + F::cast_from(0.54878743191129263322e-2_f64) * t37843 - F::cast_from(0.16951189180550569635e1_f64) * t37848 - F::cast_from(0.50853567541651708904e1_f64) * t37851;
    t41537
}
