//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 829/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk829<F: Float>(t1548: F, t775: F, t800: F, t4365: F, t837: F, t4364: F, t125: F, t1544: F, t2747: F, t1549: F, t2703: F, t124: F, t4343: F) -> (F, F, F, F, F, F) {
    let t4442 = t800 * t1548 * t775;
    let t4446 = t4365 * t837;
    let t4447 = t4364 * t4446;
    let t4450 = t125 * t1544;
    let t4451 = t4450 * t837;
    let t4452 = t2747 * t4451;
    let t4455 = t2703 * t1549;
    let t4457 = t124 * t4343;
    (t4442, t4447, t4450, t4452, t4455, t4457)
}
