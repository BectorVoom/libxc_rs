//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2873/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2873<F: Float>(t40945: F, t40958: F, t4366: F, t4504: F, t51660: F, t51676: F, t51683: F, t51685: F, t51686: F, t51688: F, t51704: F, t63015: F, t76131: F) -> F {
    let t77289 = F::cast_from(0.58911598146606471821e-3_f64) * t51660 - F::cast_from(0.58911598146606471821e-3_f64) * t51676 - F::cast_from(0.29272321618148349057e-1_f64) * t63015 + t51683 - t51685 + F::cast_from(0.51220160311720645768e-1_f64) * t51686 + F::cast_from(0.19514881078765566038e-2_f64) * t51688 - F::cast_from(0.46263278077393568556e-2_f64) * t40945 - F::cast_from(0.17073386770573548589e-1_f64) * t40958 - t51704 + F::cast_from(0.11853808529283920877e2_f64) * t4504 * t76131 * t4366;
    t77289
}
