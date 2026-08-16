//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1074/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1074<F: Float>(t30852: F, t4406: F, t1312: F, t2059: F, t21651: F, t8335: F, t30153: F, t4391: F, t6505: F, t8398: F, t6204: F, t14995: F) -> (F, F, F, F, F) {
    let t31639 = t4406 * t30852;
    let t31640 = t1312 * t31639;
    let t31644 = t21651 * t2059 * t8335;
    let t31645 = t1312 * t31644;
    let t31651 = t4391 * t30153;
    let t31652 = t1312 * t31651;
    let t31655 = t6505 * t8398;
    let t31656 = t6204 * t31655;
    let t31659 = t14995 * t30153;
    (t31640, t31645, t31652, t31656, t31659)
}
