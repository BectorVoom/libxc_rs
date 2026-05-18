//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 524/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk524<F: Float>(t1856: F, t72: F, t757: F, t539: F, t73: F, t1412: F, t1868: F, t1883: F, t221: F, t4019: F, t4018: F, t241: F, t4000: F, t820: F) -> (F, F, F, F, F, F, F) {
    let t5635 = t1856 * t72;
    let t5636 = t5635 * t757;
    let t5650 = t539 * t73;
    let t5651 = t1412 * t1868;
    let t5665 = t4019 * t221 * t1883;
    let t5666 = t4018 * t5665;
    let t5671 = t820 * t4000 * t241;
    (t5635, t5636, t5650, t5651, t5665, t5666, t5671)
}
