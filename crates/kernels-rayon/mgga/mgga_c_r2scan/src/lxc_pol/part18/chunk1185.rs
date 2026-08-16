//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1185/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1185(t1577: f64, t3308: f64, t9529: f64, t10781: f64, t9254: f64, t37782: f64, t8774: f64, t11811: f64, t39378: f64, t3179: f64, t3316: f64, t43144: f64, t43146: f64, t43149: f64, t43151: f64, t43153: f64, t43155: f64, t43157: f64) -> f64 {
    let t43160 = t1577 * t3308 * t9529;
    let t43162 = t10781 * t9254;
    let t43165 = t37782 * t3308 * t8774;
    let t43167 = t39378 * t11811;
    let t43169 = t3179 * t3316;
    let t43171 = 0.10975748638225852664e0_f64 * t43144 - 0.16463622957338778997e0_f64 * t43146 - 0.2600466522016280569e0_f64 * t43149 + 0.86682217400542685632e-1_f64 * t43151 + 0.54878743191129263322e-1_f64 * t43153 - 0.27439371595564631661e-1_f64 * t43155 - 0.16463622957338778997e0_f64 * t43157 + 0.86682217400542685632e-1_f64 * t43160 + 0.10975748638225852664e0_f64 * t43162 - 0.86682217400542685632e-1_f64 * t43165 + 0.2600466522016280569e0_f64 * t43167 - 0.11557628986739024751e0_f64 * t43169;
    t43171
}
