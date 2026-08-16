//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1256/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1256(t11479: f64, t11497: f64, t3262: f64, t38211: f64, t38226: f64, t38229: f64, t38234: f64, t38245: f64, t38251: f64, t38259: f64, t38261: f64, t38265: f64, t38268: f64, t38270: f64, t40604: f64, t43801: f64, t43805: f64) -> (f64, f64) {
    let t43929 = 3.0_f64 / 2.0_f64 * t3262 * t11479 * t11497;
    let t43932 = 0.30487649791575028314e-3_f64 * t38211 - t38226 - t38229 + t38234 + t40604 - t38245 - 0.8081505494844540645e-6_f64 * t38251 + t43801 + t43805 + t43929 - 0.15243824895787514157e-3_f64 * t38259 + 0.15243824895787514157e-3_f64 * t38261 - t38265 - t38268 - t38270;
    (t43929, t43932)
}
