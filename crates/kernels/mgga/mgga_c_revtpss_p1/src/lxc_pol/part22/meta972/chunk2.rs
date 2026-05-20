//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3254/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3254<F: Float>(t14663: F, t2745: F, t40455: F, t40473: F, t40475: F, t40477: F, t40489: F, t4364: F, t4365: F, t50472: F, t50493: F, t50497: F, t50502: F, t50504: F) -> F {
    let t61748 = F::cast_from(0.40015750243531754508e-2_f64) * t50472 - F::cast_from(0.42874018118069736972e-3_f64) * t2745 * t4364 * t4365 * t14663 - F::cast_from(0.16065646176094875955e-5_f64) * t40455 - F::cast_from(0.76220476654346199061e-4_f64) * t40473 - F::cast_from(0.76220476654346199061e-4_f64) * t40475 + F::cast_from(0.54208002996571016772e-3_f64) * t40477 + F::cast_from(0.14450132032386466905e-2_f64) * t40489 - F::cast_from(0.28582678745379824648e-4_f64) * t50493 + F::cast_from(0.85748036236139473944e-4_f64) * t50497 + F::cast_from(0.28582678745379824648e-3_f64) * t50502 - F::cast_from(0.30488190661738479624e-3_f64) * t50504;
    t61748
}
