//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 963/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk963(t12279: f64, t12284: f64, t12286: f64, t12291: f64, t12293: f64, t12297: f64, t12301: f64, t12305: f64, t12308: f64, t12310: f64, t12313: f64, t12348: f64, t12390: f64, t12432: f64, t1315: f64, t1363: f64, t3790: f64, t3795: f64, t5246: f64) -> f64 {
    let t12434 = t5246 * t12279 / 512.0_f64 - 7.0_f64 / 192.0_f64 * t12284 + t12286 * t3795 / 512.0_f64 - t12291 * t12293 / 512.0_f64 + t3790 * t12297 / 512.0_f64 + 7.0_f64 / 768.0_f64 * t12301 + 5.0_f64 / 256.0_f64 * t1363 * t12305 - 35.0_f64 / 72.0_f64 * t12308 + 7.0_f64 / 48.0_f64 * t12310 - t1315 * t12313 / 48.0_f64 + t12348 + t12390 + t12432;
    t12434
}
