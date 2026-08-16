//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1168/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1168<F: Float>(t1219: F, t4487: F, t10111: F, t10204: F, t1233: F, t1260: F, t12892: F, t12958: F, t13032: F, t13059: F, t13063: F, t13067: F, t13094: F, t1640: F, t220: F, t3261: F, t3327: F, t3332: F, t3374: F, t339: F, t4417: F, t4460: F, t4498: F, t4499: F, t4508: F, t4511: F, t523: F) -> F {
    let t13098 = t1219 * t4487;
    let t13108 = F::cast_from(2.0_f64) * t10111 * t4498 * t4499 - t10204 * t1640 * t339 - F::cast_from(2.0_f64) * t1233 * t13063 * t4508 - F::cast_from(2.0_f64) * t1233 * t13067 * t4508 - F::cast_from(2.0_f64) * t1233 * t13098 * t339 - t1260 * t12958 * t339 - F::cast_from(6.0_f64) * t12892 * t13059 * t4499 + t13032 * t220 * t523 + F::cast_from(4.0_f64) * t13063 * t4417 * t4498 + F::cast_from(4.0_f64) * t13067 * t4417 * t4498 + F::cast_from(2.0_f64) * t13094 * t3261 * t339 + F::cast_from(6.0_f64) * t3261 * t4498 * t4499 - t3327 * t339 * t4511 - t3327 * t4499 * t4508 - t3332 * t339 * t4511 - t3332 * t4499 * t4508 - F::cast_from(2.0_f64) * t3374 * t339 * t4460;
    t13108
}
