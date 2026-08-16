//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2113/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2113<F: Float>(t5775: F, t689: F, t7242: F, t25898: F, t98040: F, t25901: F, t25878: F, t98356: F, t27989: F, t94921: F, t13747: F, t1882: F, t25930: F, t27980: F, t7279: F, t94898: F, t94902: F, t94904: F, t98358: F, t98360: F, t98362: F, t98368: F, t98372: F, t98376: F) -> F {
    let t98379 = F::cast_from(0.10975748638225852664e-1_f64) * t689 * t7242 * t5775;
    let t98380 = t98040 * t25898;
    let t98382 = F::cast_from(0.25702851531048074406e-1_f64) * t98380 * t25901;
    let t98384 = F::cast_from(0.51405703062096148812e-1_f64) * t25878 * t98356;
    let t98387 = F::cast_from(0.14456046980341999104e-1_f64) * t94921 * t27989;
    let t98388 = -t98358 - t98360 + F::cast_from(0.34694512752820797848e1_f64) * t25930 * t27980 * t1882 * t98362 + F::cast_from(0.54878743191129263322e-2_f64) * t94898 - t98368 + F::cast_from(0.19514881078765566038e-1_f64) * t94902 + F::cast_from(0.26341796731742046394e1_f64) * t7279 * t13747 - F::cast_from(0.13009920719177044025e-1_f64) * t98372 + t98376 + t98379 + t98382 + t98384 + F::cast_from(0.14456046980341999104e-1_f64) * t94904 - t98387;
    t98388
}
