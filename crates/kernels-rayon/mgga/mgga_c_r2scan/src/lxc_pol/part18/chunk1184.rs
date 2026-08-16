//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1184/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1184(t10781: f64, t9258: f64, t3295: f64, t9536: f64, t3308: f64, t6362: f64, t9543: f64, t11808: f64, t39375: f64, t8849: f64, t8853: f64, t11670: f64, t8844: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t43144 = t10781 * t9258;
    let t43146 = t3295 * t9536;
    let t43149 = t6362 * t3308 * t9543;
    let t43151 = t39375 * t11808;
    let t43153 = t10781 * t8849;
    let t43155 = t3295 * t8853;
    let t43157 = t11670 * t8844;
    (t43144, t43146, t43149, t43151, t43153, t43155, t43157)
}
