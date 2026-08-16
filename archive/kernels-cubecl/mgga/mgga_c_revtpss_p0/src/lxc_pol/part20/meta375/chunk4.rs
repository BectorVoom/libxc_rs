//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1362/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1362<F: Float>(t40406: F, t685: F, t827: F, t837: F, t10837: F, t9775: F, t10828: F, t2741: F, t10818: F, t221: F, t10703: F, t2674: F) -> (F, F, F, F) {
    let t40409 = t40406 * t827 * t685 * t837;
    let t40411 = t9775 * t10837;
    let t40413 = t2741 * t10828;
    let t40419 = t221 * t10818;
    let t40421 = t2674 * t10703 * t40419;
    (t40409, t40411, t40413, t40421)
}
