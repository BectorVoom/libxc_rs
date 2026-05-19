//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1217/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1217<F: Float>(t10985: F, t11629: F, t3275: F, t38211: F, t38216: F, t38220: F, t38226: F, t38229: F, t38234: F, t38245: F, t38251: F, t40587: F, t40592: F, t40598: F, t40604: F, t40606: F, t40612: F) -> (F, F) {
    let t40615 = F::new(5.0) / F::new(8.0) * t3275 * t11629 * t10985;
    let t40616 = -F::cast_from(0.80815054948445406448e-6_f64) * t40587 + t40592 + t40598 + F::cast_from(0.60975299583150056628e-3_f64) * t38211 - F::cast_from(0.15243824895787514157e-3_f64) * t38216 + F::cast_from(0.21684485328539747656e-4_f64) * t38220 - t38226 - t38229 + t38234 + t40604 + t40606 - t38245 - F::cast_from(0.1616301098968908129e-5_f64) * t38251 + t40612 - t40615;
    (t40615, t40616)
}
