//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1492/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1492<F: Float>(t1437: F, t2482: F, t6843: F, t136: F, t2457: F, t3964: F, t6888: F, t10073: F, t22365: F, t22373: F, t10069: F, t22369: F) -> (F, F, F, F, F) {
    let t74892 = t2482 * t1437 * t6843;
    let t74901 = t3964 * t6888 * t136 * t2457;
    let t74945 = t10073 * t22365;
    let t74990 = t10073 * t22373;
    let t74999 = t10069 * t22369;
    (t74892, t74901, t74945, t74990, t74999)
}
