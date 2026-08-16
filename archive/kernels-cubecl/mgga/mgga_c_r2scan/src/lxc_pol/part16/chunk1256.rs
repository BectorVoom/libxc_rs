//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1256/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1256<F: Float>(t11479: F, t11497: F, t3262: F, t38211: F, t38226: F, t38229: F, t38234: F, t38245: F, t38251: F, t38259: F, t38261: F, t38265: F, t38268: F, t38270: F, t40604: F, t43801: F, t43805: F) -> (F, F) {
    let t43929 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t3262 * t11479 * t11497;
    let t43932 = F::cast_from(0.30487649791575028314e-3_f64) * t38211 - t38226 - t38229 + t38234 + t40604 - t38245 - F::cast_from(0.8081505494844540645e-6_f64) * t38251 + t43801 + t43805 + t43929 - F::cast_from(0.15243824895787514157e-3_f64) * t38259 + F::cast_from(0.15243824895787514157e-3_f64) * t38261 - t38265 - t38268 - t38270;
    (t43929, t43932)
}
