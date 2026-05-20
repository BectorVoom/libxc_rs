//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1924/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1924<F: Float>(t29524: F, t72: F, t1927: F, t7715: F, t7719: F, t5868: F, t76: F, t1926: F, t1470: F, t4173: F, t1493: F, t1497: F) -> (F, F, F, F, F, F, F) {
    let t29525 = t29524 * t72;
    let t29526 = t29525 * t1927;
    let t29529 = t7715 * t7719;
    let t29532 = t76 * t5868;
    let t29533 = t1926 * t29532;
    let t29538 = t4173 * t1470;
    let t29543 = t1493 * t1497;
    (t29525, t29526, t29529, t29532, t29533, t29538, t29543)
}
