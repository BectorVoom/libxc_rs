//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1216/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1216<F: Float>(t37835: F, t37838: F, t37841: F, t37843: F, t37848: F, t37851: F, t38528: F, t38532: F, t39740: F, t39742: F, t39746: F, t39749: F) -> F {
    let t41537 = t38528 + t38532 + F::new(0.11708928647259339623e0) * t37835 + F::new(0.90044238659382329742e0) * t37838 + F::new(0.27013271597814698923e1) * t37841 - F::new(0.17336443480108537126e0) * t39740 - F::new(0.86682217400542685632e-1) * t39742 - F::new(0.5200933044032561138e0) * t39746 + F::new(0.26198215989259945076e-1) * t39749 + F::new(0.54878743191129263322e-2) * t37843 - F::new(0.16951189180550569635e1) * t37848 - F::new(0.50853567541651708904e1) * t37851;
    t41537
}
