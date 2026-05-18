//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1148/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1148<F: Float>(t6866: F, t6892: F, t1721: F, t600: F, t7084: F, t1719: F, t2639: F, t164: F, t5257: F, t6877: F, t6904: F, t2575: F) -> (F, F, F, F, F, F, F) {
    let t19958 = t6892 * t6866;
    let t19961 = t7084 * t1721 * t600;
    let t19965 = t2639 * t1719;
    let t19966 = t19965 * t164;
    let t19970 = t5257 * t6877;
    let t19972 = t6892 * t6904;
    let t19974 = t2575 * t1719;
    (t19958, t19961, t19965, t19966, t19970, t19972, t19974)
}
