//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2789/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2789<F: Float>(t14141: F, t14143: F, t5658: F, t676: F, t22252: F, t555: F, t1419: F, t6843: F, t14224: F, t14238: F, t2782: F, t6861: F) -> (F, F, F, F, F) {
    let t74949 = t14141 * t14143 * t676 * t5658;
    let t74965 = t555 * t22252;
    let t74973 = t1419 * t6843;
    let t74979 = t2782 * t14238 * t14224;
    let t74982 = t1419 * t6861;
    (t74949, t74965, t74973, t74979, t74982)
}
