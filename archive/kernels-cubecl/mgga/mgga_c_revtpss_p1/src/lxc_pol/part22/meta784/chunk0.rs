//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2874/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2874<F: Float>(t3555: F, t5462: F, t5477: F, t1209: F, t17948: F, t12050: F, t471: F, t3588: F, t3552: F, t3781: F, t1204: F, t13147: F) -> (F, F, F, F, F, F) {
    let t45715 = t3555 * t5462;
    let t45718 = t3555 * t5477;
    let t45738 = t1209 * t17948;
    let t45739 = t12050 * t471;
    let t45744 = t45739 * t3588;
    let t45764 = t3552 * t3781;
    let t45769 = t1204 * t13147;
    (t45715, t45718, t45738, t45744, t45764, t45769)
}
