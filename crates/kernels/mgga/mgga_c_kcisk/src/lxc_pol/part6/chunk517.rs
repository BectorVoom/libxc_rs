//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 517/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk517<F: Float>(t339: F, t63: F, t67: F, t378: F, t4143: F, t1305: F, t2160: F, t1308: F, t2159: F) -> (F, F, F, F) {
    let t6141 = t339 * t63 * t67;
    let t6142 = t378 * t4143;
    let t6155 = t2160 * t1305;
    let t6157 = t2159 * t1308;
    (t6141, t6142, t6155, t6157)
}
