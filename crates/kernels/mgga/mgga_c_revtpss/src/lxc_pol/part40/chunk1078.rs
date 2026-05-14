//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1078/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1078<F: Float>(t9858: F, t9861: F, t2619: F, t5635: F, t13664: F, t13667: F, t13669: F, t13671: F, t13673: F, t13682: F, t13683: F, t9524: F, t9542: F, t9588: F, t9854: F, t9865: F, t9868: F) -> (F, F, F, F) {
    let t13885 = 0.34631718211362927518e2 * t9858;
    let t13886 = 0.21687162600603479684e-1 * t9861;
    let t13887 = t5635 * t2619;
    let t13888 = 0.24415263074675393405e-3 * t13887;
    let t13889 = -t9588 - t9524 - t13664 + t13667 + t13669 - t13671 + t13673 + t9542 + t13682 - t9854 + t13683 - t13885 + t13886 + t9865 + t9868 + t13888;
    (t13885, t13886, t13888, t13889)
}
