//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 887/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk887<F: Float>(t24898: F, t7105: F, t15369: F, t29055: F, t7114: F, t15460: F, t1091: F, t34197: F, t2881: F, t1901: F, t36114: F, t36118: F, t36123: F, t36127: F, t36130: F, t36135: F, t36138: F, t36142: F, t36145: F, t446: F) -> (F, F, F, F, F, F, F) {
    let t36148 = t24898 * t7105;
    let t36149 = t15369 * t36148;
    let t36152 = t29055 * t7114;
    let t36153 = t15460 * t36152;
    let t36156 = t34197 * t1091;
    let t36157 = t2881 * t36156;
    let t36160 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t36114 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t36118 + t446 * t36123 / F::cast_from(3.0_f64) + t1901 * t36127 / F::cast_from(9.0_f64) + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t36130 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t36135 - F::cast_from(2.0_f64) * t446 * t36138 + t1901 * t36142 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t36145 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t36149 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t36153 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t36157;
    (t36148, t36149, t36152, t36153, t36156, t36157, t36160)
}
