//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2007/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2007(t41040: f64, t685: f64, t28313: f64, t93317: f64, t4534: f64, t689: f64, t7384: f64, t14489: f64, t14495: f64, t1558: f64, t231: f64, t25383: f64, t25391: f64, t26473: f64, t26547: f64, t26550: f64, t27275: f64, t27353: f64, t28310: f64, t28378: f64, t28425: f64, t4487: f64, t51529: f64, t51574: f64, t51608: f64, t7070: f64, t7076: f64, t7403: f64, t7424: f64, t95825: f64, t99316: f64, t99512: f64) -> (f64, f64) {
    let t103181 = t685 * t41040;
    let t103182 = t28313 * t103181;
    let t103184 = 0.15421710918628844644e0_f64 * t93317 * t103182;
    let t103196 = 0.10975748638225852664e-1_f64 * t689 * t7384 * t4534;
    let t103210 = 0.8673628188205199462e0_f64 * t27353 * t95825 * t14495 - 0.17347256376410398924e1_f64 * t25391 * t26550 * t99512 - 0.8673628188205199462e0_f64 * t25391 * t26550 * t99316 - 0.8673628188205199462e0_f64 * t27275 * t7424 - t103184 - 0.26020884564615598386e1_f64 * t27353 * t28425 * t51574 - 0.17347256376410398924e1_f64 * t27353 * t28425 * t51529 - 0.8673628188205199462e0_f64 * t27353 * t28425 * t51608 + t103196 + 0.8673628188205199462e0_f64 * t25383 * t28378 - 0.39512695097613069591e1_f64 * t7403 * t14489 + 0.26341796731742046394e1_f64 * t26547 * t4487 + 0.4336814094102599731e0_f64 * t7070 * t7076 * t26473 * t1558 * t231 + 0.17347256376410398924e1_f64 * t25383 * t28310;
    (t103182, t103210)
}
