//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 378/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk378<F: Float>(t1240: F, t2192: F, t209: F, t9: F, t1268: F, t287: F, t421: F) -> (F, F, F, F) {
    let t2193 = t1240 * t2192;
    let t2194 = t209 * t9;
    let t2196 = t287 * t421 * t1268;
    let t2197 = t2194 * t2196;
    (t2193, t2194, t2196, t2197)
}
