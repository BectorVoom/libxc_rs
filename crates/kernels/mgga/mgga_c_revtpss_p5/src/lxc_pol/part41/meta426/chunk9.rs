//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1495/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1495<F: Float>(t1921: F, t8372: F, t31582: F, t575: F, t1913: F, t8389: F, t117781: F, t117783: F, t117789: F, t117793: F, t118502: F, t118533: F, t118576: F, t1456: F, t1458: F, t2192: F, t22533: F, t3: F, t31329: F, t31619: F, t6937: F, t8302: F) -> F {
    let t118579 = t8372 * t1921;
    let t118583 = t31582 * t575;
    let t118585 = t1913 * t8389;
    let t118587 = t6937 * t8302 + t22533 * t2192 + F::cast_from(2.0_f64) * t31329 * t1921 + t1458 * (t118533 + t118576) + F::cast_from(2.0_f64) * t118579 + t117781 + t3 * t118502 * t575 + t117783 + t117789 + t117793 + t118583 + t1456 * t31619 + F::cast_from(2.0_f64) * t118585;
    t118587
}
