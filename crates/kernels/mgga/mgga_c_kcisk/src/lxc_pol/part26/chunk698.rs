//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 698/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk698<F: Float>(t8071: F, t8184: F, t504: F, t2282: F, t6241: F) -> (F, F, F, F) {
    let t8185 = t8071 + t8184;
    let t8186 = t8185 * t504;
    let t8188 = 2.0 * t6241 * t2282;
    let t8189 = t2282 * t2282;
    (t8185, t8186, t8188, t8189)
}
