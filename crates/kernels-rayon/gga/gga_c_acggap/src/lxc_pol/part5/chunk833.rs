//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 833/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk833(t2773: f64, t286: f64, t680: f64, t800: f64, t2632: f64, t883: f64, t2631: f64, t2787: f64, t686: f64, t2896: f64, t98: f64, t100: f64, t2908: f64) -> (f64, f64, f64, f64, f64) {
    let t11596 = 0.62337092780453269531e3_f64 * t286 * t2773 * t680 * t800;
    let t11597 = t883 * t2632;
    let t11602 = 0.69263436422725855036e2_f64 * t286 * t686 * t2787 * t2631;
    let t11607 = 1.0_f64 / t98 / t2896;
    let t11627 = 1.0_f64 / t100 / t2908;
    (t11596, t11597, t11602, t11607, t11627)
}
