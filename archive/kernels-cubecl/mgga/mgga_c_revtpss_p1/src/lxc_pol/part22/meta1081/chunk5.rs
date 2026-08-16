//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3899/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3899<F: Float>(t1437: F, t2482: F, t6843: F, t4104: F, t136: F, t2457: F, t3964: F, t6888: F, t1882: F, t5767: F, t1892: F, t5658: F) -> (F, F, F, F) {
    let t74892 = t2482 * t1437 * t6843;
    let t74893 = t74892 * t4104;
    let t74901 = t3964 * t6888 * t136 * t2457;
    let t74908 = t2482 * t5767 * t1882 * t4104;
    let t74922 = t1892 * t5658;
    (t74893, t74901, t74908, t74922)
}
