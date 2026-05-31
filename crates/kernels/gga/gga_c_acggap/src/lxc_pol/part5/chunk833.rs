//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 833/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk833<F: Float>(t2773: F, t286: F, t680: F, t800: F, t2632: F, t883: F, t2631: F, t2787: F, t686: F, t2896: F, t98: F, t100: F, t2908: F) -> (F, F, F, F, F) {
    let t11596 = F::cast_from(0.62337092780453269531e3_f64) * t286 * t2773 * t680 * t800;
    let t11597 = t883 * t2632;
    let t11602 = F::cast_from(0.69263436422725855036e2_f64) * t286 * t686 * t2787 * t2631;
    let t11607 = F::cast_from(1.0_f64) / t98 / t2896;
    let t11627 = F::cast_from(1.0_f64) / t100 / t2908;
    (t11596, t11597, t11602, t11607, t11627)
}
