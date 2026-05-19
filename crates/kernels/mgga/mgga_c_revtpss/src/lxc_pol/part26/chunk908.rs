//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 908/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk908<F: Float>(t11610: F, t981: F, t11572: F, t300: F, t11467: F, t11506: F, t11509: F, t11114: F, t11118: F, t11530: F, t11533: F, t11547: F, t11596: F, t11600: F, t11604: F, t11608: F) -> (F, F, F, F) {
    let t11612 = F::cast_from(0.5848223622634646207e0_f64) * t981 * t11610;
    let t11614 = F::cast_from(0.19751673498613801407e-1_f64) * t300 * t11572;
    let t11616 = t11506 * t11467 * t11509;
    let t11618 = F::cast_from(0.10254018858216406658e4_f64) * t981 * t11616;
    let t11619 = t11596 - t11600 + t11604 + t11608 - t11612 + t11614 - t11547 - t11618 - t11530 + t11533 - t11114 + t11118;
    (t11612, t11614, t11618, t11619)
}
