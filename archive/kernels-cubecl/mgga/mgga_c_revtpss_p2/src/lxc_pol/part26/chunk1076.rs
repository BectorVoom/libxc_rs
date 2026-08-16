//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1076/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1076<F: Float>(t2097: F, t25924: F, t4077: F, t2027: F, t213: F, t25921: F, t25930: F, t26294: F, t26295: F, t26302: F, t26305: F, t26309: F, t26335: F, t26338: F, t26343: F, t26347: F, t26351: F, t26356: F, t26361: F, t26363: F, t26365: F, t26366: F, t4078: F, t561: F, t7295: F, t7511: F, t7523: F, t7528: F) -> (F, F) {
    let t26371 = t25924 * t2097 * t4077;
    let t26374 = -t26294 + F::cast_from(0.51405703062096148812e-1_f64) * t26295 + F::cast_from(0.13170898365871023197e1_f64) * t7511 * t4078 + F::cast_from(0.8673628188205199462e0_f64) * t25921 * t7528 + F::cast_from(0.10975748638225852664e-1_f64) * t26302 - F::cast_from(0.17347256376410398924e1_f64) * t25930 * t26305 + t26309 - F::cast_from(0.4336814094102599731e0_f64) * t2027 * t26335 + F::cast_from(0.65854491829355115987e0_f64) * t213 * t26338 * t561 - F::cast_from(0.8673628188205199462e0_f64) * t7295 * t26343 + F::cast_from(0.17347256376410398924e1_f64) * t7295 * t26347 + F::cast_from(0.8673628188205199462e0_f64) * t7295 * t26351 - F::cast_from(0.10975748638225852664e-1_f64) * t26356 - t26361 + t26363 - t26365 + F::cast_from(0.14456046980341999104e-1_f64) * t26366 + F::cast_from(0.17347256376410398924e1_f64) * t25921 * t7523 - F::cast_from(0.26020884564615598386e1_f64) * t7295 * t26371;
    (t26371, t26374)
}
