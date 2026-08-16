//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1755/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1755<F: Float>(t17471: F, t5047: F, t1222: F, t1012: F, t13026: F, t1263: F, t5245: F, t1234: F, t5390: F) -> (F, F, F, F, F) {
    let t17472 = t17471 * t5047;
    let t17474 = t1222 * t17472 / F::cast_from(324.0_f64);
    let t17475 = t1012 * t13026;
    let t17500 = t1263 * t5245;
    let t17505 = t1234 * t5390;
    (t17472, t17474, t17475, t17500, t17505)
}
