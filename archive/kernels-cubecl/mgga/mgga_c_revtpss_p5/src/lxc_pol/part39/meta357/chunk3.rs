//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1228/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1228<F: Float>(t14676: F, t4364: F, t837: F, t2646: F, t4365: F, t136: F, t243: F, t220: F, t14671: F, t10777: F, t125: F, t4343: F) -> (F, F, F, F, F) {
    let t14678 = t4364 * t14676 * t837;
    let t14682 = t4364 * t4365 * t2646;
    let t14685 = t243 * t136;
    let t14686 = t14685 * t220;
    let t14688 = t14686 * t14671 * t837;
    let t14690 = F::cast_from(0.25410001404642664112e-4_f64) * t10777 * t14688;
    let t14691 = t125 * t4343;
    (t14678, t14682, t14686, t14690, t14691)
}
