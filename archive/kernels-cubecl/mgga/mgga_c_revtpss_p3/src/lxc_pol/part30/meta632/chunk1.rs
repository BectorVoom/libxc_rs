//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2199/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2199<F: Float>(t101455: F, t1513: F, t2340: F, t94982: F, t4287: F, t665: F, t25826: F, t2366: F, t13509: F, t6998: F, t101448: F, t101451: F, t101454: F, t94974: F, t94979: F, t94981: F) -> F {
    let t101456 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t101455;
    let t101457 = t1513 * t2340;
    let t101458 = t94982 * t101457;
    let t101460 = t4287 * t665;
    let t101461 = t25826 * t101460;
    let t101463 = t1513 * t2366;
    let t101464 = t25826 * t101463;
    let t101466 = t6998 * t13509;
    let t101468 = -t94974 - t101448 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t94979 + t94981 / F::cast_from(3.0_f64) - F::cast_from(11.0_f64) / F::cast_from(9.0_f64) * t101451 - t101454 + t101456 - F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t101458 + t101461 / F::cast_from(2.0_f64) + t101464 / F::cast_from(4.0_f64) - t101466 / F::cast_from(8.0_f64);
    t101468
}
