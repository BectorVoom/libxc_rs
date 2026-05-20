//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2007/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2007<F: Float>(t41040: F, t685: F, t28313: F, t93317: F, t4534: F, t689: F, t7384: F, t14489: F, t14495: F, t1558: F, t231: F, t25383: F, t25391: F, t26473: F, t26547: F, t26550: F, t27275: F, t27353: F, t28310: F, t28378: F, t28425: F, t4487: F, t51529: F, t51574: F, t51608: F, t7070: F, t7076: F, t7403: F, t7424: F, t95825: F, t99316: F, t99512: F) -> (F, F) {
    let t103181 = t685 * t41040;
    let t103182 = t28313 * t103181;
    let t103184 = F::cast_from(0.15421710918628844644e0_f64) * t93317 * t103182;
    let t103196 = F::cast_from(0.10975748638225852664e-1_f64) * t689 * t7384 * t4534;
    let t103210 = F::cast_from(0.8673628188205199462e0_f64) * t27353 * t95825 * t14495 - F::cast_from(0.17347256376410398924e1_f64) * t25391 * t26550 * t99512 - F::cast_from(0.8673628188205199462e0_f64) * t25391 * t26550 * t99316 - F::cast_from(0.8673628188205199462e0_f64) * t27275 * t7424 - t103184 - F::cast_from(0.26020884564615598386e1_f64) * t27353 * t28425 * t51574 - F::cast_from(0.17347256376410398924e1_f64) * t27353 * t28425 * t51529 - F::cast_from(0.8673628188205199462e0_f64) * t27353 * t28425 * t51608 + t103196 + F::cast_from(0.8673628188205199462e0_f64) * t25383 * t28378 - F::cast_from(0.39512695097613069591e1_f64) * t7403 * t14489 + F::cast_from(0.26341796731742046394e1_f64) * t26547 * t4487 + F::cast_from(0.4336814094102599731e0_f64) * t7070 * t7076 * t26473 * t1558 * t231 + F::cast_from(0.17347256376410398924e1_f64) * t25383 * t28310;
    (t103182, t103210)
}
