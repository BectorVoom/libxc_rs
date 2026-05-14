//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1215/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1215<F: Float>(t3138: F, t3139: F, t9355: F, t1035: F, t3174: F, t967: F, t3233: F, t1049: F, t206: F, t1032: F, t213: F, t12651: F, t167: F, t1001: F, t31857: F, t982: F) -> (F, F, F, F, F, F, F) {
    let t110876 = t3138 * t9355 * t3139;
    let t110879 = t1035 * t967 * t3174;
    let t110881 = t3233 * t967;
    let t110883 = t206 * t1049;
    let t110885 = t1032 * t213;
    let t110887 = t12651 * t167;
    let t110890 = t982 * t31857 * t1001;
    (t110876, t110879, t110881, t110883, t110885, t110887, t110890)
}
