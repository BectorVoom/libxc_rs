//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1182/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1182<F: Float>(t4343: F, t828: F, t855: F, t1544: F, t221: F, t2675: F, t2674: F, t1558: F, t243: F) -> (F, F, F, F) {
    let t4345 = t855 * t828 * t4343;
    let t4349 = t2675 * t221 * t1544;
    let t4350 = t2674 * t4349;
    let t4352 = t243 * t1558;
    (t4345, t4349, t4350, t4352)
}
