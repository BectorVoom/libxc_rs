//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1019/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1019<F: Float>(t2100: F, t5945: F, t2099: F, t5725: F, t5730: F, t5915: F, t757: F, t5722: F, t768: F, t5954: F, t5957: F, t5933: F, t5935: F, t2003: F, t67: F, t154: F, t276: F, t5635: F) -> (F, F, F, F, F, F, F) {
    let t18142 = t5945 * t2100;
    let t18145 = t5725 * t2099 * t5730;
    let t18150 = t757 * t2099 * t5915;
    let t18152 = t768 * t5722;
    let t18158 = t5954 * t2099 * t5957;
    let t18167 = t5933 * t2099 * t5935;
    let t18182 = t67 * t2003;
    let t18185 = t276 * t154 * t18182 * t5635;
    (t18142, t18145, t18150, t18152, t18158, t18167, t18185)
}
