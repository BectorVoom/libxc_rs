//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1083/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1083<F: Float>(t40603: F, t11004: F, t11523: F, t6897: F, t983: F, t2330: F, t3275: F, t3276: F, t10985: F, t11629: F, t38211: F, t38216: F, t38220: F, t38226: F, t38229: F, t38234: F, t38245: F, t38251: F, t40587: F, t40592: F, t40598: F) -> (F, F, F, F) {
    let t40604 = 0.19211284388664477842e-2 * t40603;
    let t40606 = 5.0 / 8.0 * t11523 * t11004;
    let t40608 = t6897 * t983;
    let t40609 = t40608 * t2330;
    let t40612 = 5.0 / 8.0 * t3275 * t3276 * t40609;
    let t40615 = 5.0 / 8.0 * t3275 * t11629 * t10985;
    let t40616 = -0.80815054948445406448e-6 * t40587 + t40592 + t40598 + 0.60975299583150056628e-3 * t38211 - 0.15243824895787514157e-3 * t38216 + 0.21684485328539747656e-4 * t38220 - t38226 - t38229 + t38234 + t40604 + t40606 - t38245 - 0.1616301098968908129e-5 * t38251 + t40612 - t40615;
    (t40606, t40612, t40615, t40616)
}
