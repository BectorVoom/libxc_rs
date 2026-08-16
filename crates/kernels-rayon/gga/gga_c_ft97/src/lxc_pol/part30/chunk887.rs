//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 887/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk887(t24898: f64, t7105: f64, t15369: f64, t29055: f64, t7114: f64, t15460: f64, t1091: f64, t34197: f64, t2881: f64, t1901: f64, t36114: f64, t36118: f64, t36123: f64, t36127: f64, t36130: f64, t36135: f64, t36138: f64, t36142: f64, t36145: f64, t446: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t36148 = t24898 * t7105;
    let t36149 = t15369 * t36148;
    let t36152 = t29055 * t7114;
    let t36153 = t15460 * t36152;
    let t36156 = t34197 * t1091;
    let t36157 = t2881 * t36156;
    let t36160 = 2.0_f64 / 3.0_f64 * t446 * t36114 + 2.0_f64 / 3.0_f64 * t446 * t36118 + t446 * t36123 / 3.0_f64 + t1901 * t36127 / 9.0_f64 + 4.0_f64 / 3.0_f64 * t446 * t36130 - 2.0_f64 / 3.0_f64 * t446 * t36135 - 2.0_f64 * t446 * t36138 + t1901 * t36142 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t1901 * t36145 - 4.0_f64 / 3.0_f64 * t1901 * t36149 - 4.0_f64 / 3.0_f64 * t1901 * t36153 - 2.0_f64 / 9.0_f64 * t1901 * t36157;
    (t36148, t36149, t36152, t36153, t36156, t36157, t36160)
}
