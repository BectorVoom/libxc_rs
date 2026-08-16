//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1168/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1168(t1219: f64, t4487: f64, t10111: f64, t10204: f64, t1233: f64, t1260: f64, t12892: f64, t12958: f64, t13032: f64, t13059: f64, t13063: f64, t13067: f64, t13094: f64, t1640: f64, t220: f64, t3261: f64, t3327: f64, t3332: f64, t3374: f64, t339: f64, t4417: f64, t4460: f64, t4498: f64, t4499: f64, t4508: f64, t4511: f64, t523: f64) -> f64 {
    let t13098 = t1219 * t4487;
    let t13108 = 2.0_f64 * t10111 * t4498 * t4499 - t10204 * t1640 * t339 - 2.0_f64 * t1233 * t13063 * t4508 - 2.0_f64 * t1233 * t13067 * t4508 - 2.0_f64 * t1233 * t13098 * t339 - t1260 * t12958 * t339 - 6.0_f64 * t12892 * t13059 * t4499 + t13032 * t220 * t523 + 4.0_f64 * t13063 * t4417 * t4498 + 4.0_f64 * t13067 * t4417 * t4498 + 2.0_f64 * t13094 * t3261 * t339 + 6.0_f64 * t3261 * t4498 * t4499 - t3327 * t339 * t4511 - t3327 * t4499 * t4508 - t3332 * t339 * t4511 - t3332 * t4499 * t4508 - 2.0_f64 * t3374 * t339 * t4460;
    t13108
}
