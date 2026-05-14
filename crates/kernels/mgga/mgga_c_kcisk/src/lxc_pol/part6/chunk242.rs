//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 242/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk242<F: Float>(t1161: F, t317: F, t305: F, t306: F) -> (F, F, F, F, F) {
    let t1162 = 0.17808333333333333333e-1 * t1161;
    let t1170 = t317 * t317;
    let t1171 = 1.0 / t1170;
    let t1172 = t305 * t1171;
    let t1173 = 1.0 / t306;
    (t1162, t1170, t1171, t1172, t1173)
}
