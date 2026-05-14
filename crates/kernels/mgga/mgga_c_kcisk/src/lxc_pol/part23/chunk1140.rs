//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1140/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1140<F: Float>(t3913: F, t467: F, t470: F, t454: F, t415: F, t20160: F, t9453: F) -> (F, F, F, F) {
    let t32169 = t467 * t3913 * t470;
    let t32170 = t454 * t32169;
    let t32171 = t415 * t32170;
    let t32173 = t20160 * t9453;
    (t32169, t32170, t32171, t32173)
}
