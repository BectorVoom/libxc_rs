//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2980/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2980<F: Float>(t14066: F, t545: F, t689: F, t869: F, t1398: F, t14141: F, t14143: F, t2434: F, t10049: F, t14145: F, t1882: F, t2482: F) -> (F, F, F) {
    let t49252 = t689 * t869 * t545 * t14066;
    let t49256 = t14141 * t14143 * t2434 * t1398;
    let t49260 = t2482 * t10049 * t1882 * t14145;
    (t49252, t49256, t49260)
}
