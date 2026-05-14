//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1049/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1049<F: Float>(t3308: F, t37782: F, t8774: F, t11811: F, t39378: F, t3179: F, t3316: F, t43144: F, t43146: F, t43149: F, t43151: F, t43153: F, t43155: F, t43157: F, t43160: F, t43162: F) -> (F,) {
    let t43165 = t37782 * t3308 * t8774;
    let t43167 = t39378 * t11811;
    let t43169 = t3179 * t3316;
    let t43171 = 0.10975748638225852664e0 * t43144 - 0.16463622957338778997e0 * t43146 - 0.2600466522016280569e0 * t43149 + 0.86682217400542685632e-1 * t43151 + 0.54878743191129263322e-1 * t43153 - 0.27439371595564631661e-1 * t43155 - 0.16463622957338778997e0 * t43157 + 0.86682217400542685632e-1 * t43160 + 0.10975748638225852664e0 * t43162 - 0.86682217400542685632e-1 * t43165 + 0.2600466522016280569e0 * t43167 - 0.11557628986739024751e0 * t43169;
    (t43171,)
}
