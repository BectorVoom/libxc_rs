//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1014/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1014<F: Float>(t1314: F, t507: F, t8806: F, t34406: F, t6324: F, t8463: F, t8480: F, t8652: F, t7575: F, t8514: F, t2288: F, t8791: F, t13287: F, t34823: F, t1181: F, t2068: F, t38784: F, t599: F) -> (F, F, F, F, F, F, F) {
    let t40050 = t8806 * t507 * t1314;
    let t40054 = t34406 * t6324;
    let t40057 = t8463 * t8480 * t8652;
    let t40063 = t7575 * t8480 * t8514;
    let t40066 = t2288 * t8791;
    let t40068 = t34823 * t13287 * t40066;
    let t40072 = t2068 * t1181 * t599 * t38784;
    (t40050, t40054, t40057, t40063, t40066, t40068, t40072)
}
