//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2315/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2315(t11888: f64, t1215: f64, t1244: f64, t1246: f64, t15032: f64, t1729: f64, t19156: f64, t19179: f64, t24589: f64, t27465: f64, t27516: f64, t27722: f64, t29664: f64, t29708: f64, t29712: f64, t3604: f64, t4964: f64, t6168: f64, t7373: f64, t7375: f64, t7376: f64, t7389: f64, t8083: f64, t8085: f64, t95747: f64, t95751: f64, t95758: f64) -> f64 {
    let t104002 = 0.16449340668482264365e-1_f64 * t7373 * t7375 * t19179 * t7376 + 0.54831135561607547884e-2_f64 * t24589 * t27516 * t27465 + t95747 + t3604 * t29712 + 2.0_f64 * t15032 * t8083 - t95751 + 0.48738787165873375897e-2_f64 * t95758 - 6.0_f64 * t11888 * t29708 * t19156 + 2.0_f64 * t4964 * t8085 + t1244 * t29664 * t1215 * t1246 + 2.0_f64 * t1729 * t27722 + t6168 * t7389;
    t104002
}
