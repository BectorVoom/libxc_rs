//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1272/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1272(t1142: f64, t95335: f64, t95336: f64, t95338: f64, t95340: f64, t95343: f64, t95346: f64, t95349: f64, t95352: f64, t95354: f64, t95356: f64, t95358: f64, t95386: f64, t95431: f64, t95432: f64, t95434: f64, t95436: f64, t95438: f64, t95440: f64, t95442: f64, t95444: f64, t95446: f64, t95448: f64, t95450: f64, t95477: f64) -> f64 {
    let t95481 = t1142 * (t95335 + t95336 / 48.0_f64 - t95338 / 12.0_f64 + t95340 / 8.0_f64 - t95343 / 32.0_f64 + t95346 / 27.0_f64 - t95349 / 144.0_f64 + t95352 / 8.0_f64 + t95354 / 4.0_f64 + t95356 / 3.0_f64 + t95358 / 72.0_f64 + t95386 + t95431 + t95432 / 128.0_f64 - t95434 / 96.0_f64 + t95436 / 24.0_f64 + t95438 / 64.0_f64 + t95440 / 96.0_f64 + t95442 / 54.0_f64 - t95444 / 288.0_f64 + t95446 / 4.0_f64 - t95448 / 16.0_f64 + t95450 / 128.0_f64 + t95477);
    t95481
}
