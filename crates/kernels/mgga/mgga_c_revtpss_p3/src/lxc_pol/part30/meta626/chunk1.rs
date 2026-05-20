//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2169/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2169<F: Float>(t1579: F, t2722: F, t231: F, t27266: F, t686: F, t72: F, t7058: F, t1959: F, t25391: F, t25392: F, t27353: F, t51436: F, t51698: F, t7079: F, t93242: F, t93252: F, t93262: F, t93272: F, t93273: F, t99297: F, t99300: F, t99303: F, t99307: F, t99309: F, t99313: F) -> (F, F, F) {
    let t99315 = t1579 * t2722;
    let t99316 = t99315 * t231;
    let t99321 = t27266 * t72 * t686;
    let t99323 = F::cast_from(0.14456046980341999104e-1_f64) * t7058 * t99321;
    let t99332 = -F::cast_from(0.24093411633903331839e-3_f64) * t99297 + F::cast_from(0.48186823267806663678e-3_f64) * t93242 - F::cast_from(0.4336814094102599731e0_f64) * t99300 * t1959 + F::cast_from(0.8673628188205199462e0_f64) * t99303 * t7079 + F::cast_from(0.23131639038696784278e-2_f64) * t93252 - F::cast_from(0.13009920719177044025e-1_f64) * t99307 - F::cast_from(0.8673628188205199462e0_f64) * t25391 * t25392 * t99309 - F::cast_from(0.11565819519348392139e-2_f64) * t99313 - F::cast_from(0.8673628188205199462e0_f64) * t25391 * t25392 * t99316 + t99323 + F::cast_from(0.19514881078765566038e-1_f64) * t93262 + F::cast_from(0.8673628188205199462e0_f64) * t27353 * t25392 * t51436 + F::cast_from(0.4336814094102599731e0_f64) * t27353 * t25392 * t51698 + t93272 + F::cast_from(0.2601984143835408805e-1_f64) * t93273;
    (t99315, t99321, t99332)
}
