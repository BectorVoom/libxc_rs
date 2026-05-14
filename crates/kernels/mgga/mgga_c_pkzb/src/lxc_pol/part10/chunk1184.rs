//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1184/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1184<F: Float>(t5221: F, t6916: F, t6920: F, t149: F, t5224: F, t63: F, t1020: F, t1692: F, t1769: F, t6985: F, t1041: F, t17095: F, t6866: F, t6892: F, t5257: F, t6877: F) -> (F, F, F, F, F, F, F, F) {
    let t19911 = t5221 * t6916;
    let t19913 = t5221 * t6920;
    let t19932 = t149 * t5224 * t63;
    let t19933 = t1020 * t1692;
    let t19938 = t1769 * t6985;
    let t19947 = t17095 * t1041;
    let t19958 = t6892 * t6866;
    let t19970 = t5257 * t6877;
    (t19911, t19913, t19932, t19933, t19938, t19947, t19958, t19970)
}
