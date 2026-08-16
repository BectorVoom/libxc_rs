//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1262/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1262<F: Float>(t1174: F, t18492: F, t6150: F, t2203: F, t7958: F, t836: F, t2215: F, t2209: F, t7966: F, t3041: F, t6158: F, t7972: F) -> (F, F, F, F, F, F) {
    let t22199 = t18492 * t1174 * t6150;
    let t22202 = t2203 * t7958 * t836;
    let t22205 = t2215 * t7958 * t836;
    let t22207 = t7966 * t2209;
    let t22209 = t3041 * t6158;
    let t22215 = t7972 * t2209;
    (t22199, t22202, t22205, t22207, t22209, t22215)
}
