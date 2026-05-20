//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3310/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3310<F: Float>(t10657: F, t18677: F, t39694: F, t39697: F, t39701: F, t39719: F, t39723: F, t39724: F, t39726: F, t4366: F, t4504: F, t51320: F, t51445: F, t51452: F, t51460: F, t5978: F, t61679: F, t62612: F, t62641: F, t820: F) -> F {
    let t62754 = F::cast_from(0.60712963356159538786e-1_f64) * t51445 + F::cast_from(0.65049603595885220126e-3_f64) * t39694 + t39697 + F::cast_from(0.22089088168956307394e-3_f64) * t51452 + F::cast_from(0.26341796731742046394e1_f64) * t4504 * t62641 * t4366 - F::cast_from(0.39274398764404314548e-3_f64) * t39701 - F::cast_from(0.65854491829355115987e0_f64) * t820 * t10657 * t5978 + F::cast_from(0.26341796731742046394e1_f64) * t4504 * t62612 * t4366 + F::cast_from(0.39274398764404314548e-3_f64) * t39719 - t39723 - F::cast_from(0.43902994552903410656e-1_f64) * t51460 + F::cast_from(0.52039682876708176102e-2_f64) * t39724 - F::cast_from(0.73171657588172351096e-2_f64) * t39726 + F::cast_from(0.15805078039045227836e2_f64) * t51320 * t18677 * t61679;
    t62754
}
