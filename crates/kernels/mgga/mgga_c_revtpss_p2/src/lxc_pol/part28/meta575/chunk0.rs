//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2038/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2038<F: Float>(t11858: F, t27492: F, t11926: F, t25516: F, t3114: F, t93596: F, t25577: F, t3111: F, t1020: F, t25576: F, t25490: F, t3215: F) -> (F, F, F, F, F, F) {
    let t93658 = t11858 * t27492;
    let t93667 = t11926 * t25516;
    let t93670 = t3114 * t93596;
    let t93673 = t25577 * t3111;
    let t93675 = t1020 * t25576;
    let t93683 = t25490 * t3215;
    (t93658, t93667, t93670, t93673, t93675, t93683)
}
