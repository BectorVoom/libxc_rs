//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 802/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk802<F: Float>(t1451: F, t2001: F, t1434: F, t1998: F, t1441: F, t1429: F, t1418: F, t7383: F, t7387: F, t7390: F, t7396: F, t7405: F, t8680: F, t8682: F, t8684: F, t8686: F, t8690: F) -> F {
    let t8692 = t2001 * t1451;
    let t8694 = t1998 * t1434;
    let t8696 = t2001 * t1441;
    let t8698 = t2001 * t1429;
    let t8700 = t2001 * t1418;
    let t8702 = -t7383 / F::cast_from(64.0_f64) - t7387 / F::cast_from(192.0_f64) - F::cast_from(0.7640625e-2_f64) * t7390 + F::cast_from(0.140078125e-1_f64) * t7396 + F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t7405 + F::cast_from(11.0_f64) / F::cast_from(384.0_f64) * t8680 + F::cast_from(11.0_f64) / F::cast_from(1152.0_f64) * t8682 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t8684 + F::cast_from(0.25724410870841842183e-2_f64) * t8686 - F::cast_from(0.10718504529517434243e-3_f64) * t8690 - F::cast_from(0.17149607247227894789e-2_f64) * t8692 + F::cast_from(0.85748036236139473944e-3_f64) * t8694 + F::cast_from(0.34299214494455789578e-2_f64) * t8696 + F::cast_from(0.85748036236139473945e-2_f64) * t8698 - F::cast_from(0.34299214494455789578e-2_f64) * t8700;
    t8702
}
