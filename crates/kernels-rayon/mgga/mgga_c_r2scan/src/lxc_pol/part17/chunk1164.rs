//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1164/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1164(t11808: f64, t39375: f64, t10781: f64, t8849: f64, t3295: f64, t8853: f64, t11670: f64, t8844: f64, t1577: f64, t3308: f64, t9529: f64, t9254: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43151 = t39375 * t11808;
    let t43153 = t10781 * t8849;
    let t43155 = t3295 * t8853;
    let t43157 = t11670 * t8844;
    let t43160 = t1577 * t3308 * t9529;
    let t43162 = t10781 * t9254;
    (t43151, t43153, t43155, t43157, t43160, t43162)
}
