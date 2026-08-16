//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1217/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1217(t10985: f64, t11629: f64, t3275: f64, t38211: f64, t38216: f64, t38220: f64, t38226: f64, t38229: f64, t38234: f64, t38245: f64, t38251: f64, t40587: f64, t40592: f64, t40598: f64, t40604: f64, t40606: f64, t40612: f64) -> (f64, f64) {
    let t40615 = 5.0_f64 / 8.0_f64 * t3275 * t11629 * t10985;
    let t40616 = -0.80815054948445406448e-6_f64 * t40587 + t40592 + t40598 + 0.60975299583150056628e-3_f64 * t38211 - 0.15243824895787514157e-3_f64 * t38216 + 0.21684485328539747656e-4_f64 * t38220 - t38226 - t38229 + t38234 + t40604 + t40606 - t38245 - 0.1616301098968908129e-5_f64 * t38251 + t40612 - t40615;
    (t40615, t40616)
}
