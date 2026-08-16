//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1719/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1719(t2097: f64, t25924: f64, t4077: f64, t2027: f64, t213: f64, t25921: f64, t25930: f64, t26294: f64, t26295: f64, t26302: f64, t26305: f64, t26309: f64, t26335: f64, t26338: f64, t26343: f64, t26347: f64, t26351: f64, t26356: f64, t26361: f64, t26363: f64, t26365: f64, t26366: f64, t4078: f64, t561: f64, t7295: f64, t7511: f64, t7523: f64, t7528: f64) -> (f64, f64) {
    let t26371 = t25924 * t2097 * t4077;
    let t26374 = -t26294 + 0.51405703062096148812e-1_f64 * t26295 + 0.13170898365871023197e1_f64 * t7511 * t4078 + 0.8673628188205199462e0_f64 * t25921 * t7528 + 0.10975748638225852664e-1_f64 * t26302 - 0.17347256376410398924e1_f64 * t25930 * t26305 + t26309 - 0.4336814094102599731e0_f64 * t2027 * t26335 + 0.65854491829355115987e0_f64 * t213 * t26338 * t561 - 0.8673628188205199462e0_f64 * t7295 * t26343 + 0.17347256376410398924e1_f64 * t7295 * t26347 + 0.8673628188205199462e0_f64 * t7295 * t26351 - 0.10975748638225852664e-1_f64 * t26356 - t26361 + t26363 - t26365 + 0.14456046980341999104e-1_f64 * t26366 + 0.17347256376410398924e1_f64 * t25921 * t7523 - 0.26020884564615598386e1_f64 * t7295 * t26371;
    (t26371, t26374)
}
