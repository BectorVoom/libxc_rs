//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3673/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3673<F: Float>(t20520: F, t3479: F, t1168: F, t12418: F, t12423: F, t12429: F, t12470: F, t12472: F, t12511: F, t17086: F, t20521: F, t20615: F, t20618: F, t20619: F, t20625: F, t3447: F, t3452: F, t3453: F, t3471: F, t3477: F, t45085: F, t45194: F, t5120: F, t6487: F, t6502: F, t6503: F, t6506: F, t68956: F, t68961: F, t68963: F, t68965: F, t68967: F) -> F {
    let t69411 = t20520 * t3479;
    let t69422 = -t68956 + t68961 + t68963 - t68965 - t68967 + F::new(2.0) * t5120 * t17086 - F::new(2.0) * t45194 * t6487 + F::new(1.0) * t12418 * t6503 + F::new(2.0) * t3447 * t20521 - F::cast_from(0.19298375398431042081e3_f64) * t12429 * t6506 * t3471 - F::cast_from(0.24828486201251232145e5_f64) * t45085 * t20625 * t3453 - F::new(4.0) * t12511 * t20615 - F::new(4.0) * t3452 * t20521 * t1168 - F::new(2.0) * t3452 * t6503 * t3471 - F::cast_from(0.19298375398431042081e3_f64) * t12429 * t20618 * t3453 + F::cast_from(0.64327917994770140268e2_f64) * t12423 * t20619 + F::cast_from(0.64327917994770140268e2_f64) * t3477 * t69411 * t1168 + F::cast_from(0.32163958997385070134e2_f64) * t3477 * t20618 * t3471 + F::cast_from(0.2069040516770936012e4_f64) * t12470 * t6502 * t12472 * t3453;
    t69422
}
