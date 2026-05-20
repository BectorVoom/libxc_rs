//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2002/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2002<F: Float>(t786: F, t7998: F, t867: F, t2467: F, t1580: F, t26446: F, t689: F, t28368: F, t93321: F, t93374: F, t25317: F, t26511: F, t26550: F, t26551: F, t26568: F, t26573: F, t27199: F, t27353: F, t2771: F, t28400: F, t51698: F, t7067: F, t7070: F, t7997: F, t93349: F, t95740: F, t95744: F, t95747: F, t99191: F, t99277: F) -> F {
    let t103067 = t786 * t7998 * t867;
    let t103069 = F::cast_from(0.19514881078765566038e-1_f64) * t103067 * t2467;
    let t103072 = F::cast_from(0.10975748638225852664e-1_f64) * t689 * t26446 * t1580;
    let t103086 = F::cast_from(0.14456046980341999104e-1_f64) * t93321 * t28368;
    let t103088 = F::cast_from(0.25702851531048074406e-1_f64) * t93374 * t28368;
    let t103100 = -t103069 + t103072 - F::cast_from(0.26020884564615598386e1_f64) * t7070 * t25317 * t7997 * t2771 - F::cast_from(0.48186823267806663678e-3_f64) * t95740 - F::cast_from(0.19514881078765566038e-1_f64) * t95744 + F::cast_from(0.26020884564615598386e1_f64) * t93349 * t26550 * t99277 - F::cast_from(0.8673628188205199462e0_f64) * t27199 * t26511 + F::cast_from(0.45699670022203476294e-2_f64) * t95747 - t103086 + t103088 + F::cast_from(0.4336814094102599731e0_f64) * t27353 * t26550 * t51698 + F::cast_from(0.8673628188205199462e0_f64) * t27199 * t26568 + F::cast_from(0.4336814094102599731e0_f64) * t27199 * t26573 - F::cast_from(0.8673628188205199462e0_f64) * t7067 * t28400 - F::cast_from(0.17347256376410398924e1_f64) * t99191 * t26551;
    t103100
}
