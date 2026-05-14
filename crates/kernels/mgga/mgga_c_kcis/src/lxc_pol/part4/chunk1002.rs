//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1002/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1002<F: Float>(t13663: F, t14057: F, t304: F, t355: F, t360: F, t303: F, t1750: F, t3245: F, t3209: F, t3218: F, t4813: F, t922: F, t3200: F, t1022: F, t9409: F, t4818: F) -> (F, F, F, F, F, F) {
    let t14058 = t13663 + t14057;
    let t14059 = t304 * t14058;
    let t14060 = t14059 * t355;
    let t14061 = t14060 * t360;
    let t14062 = t303 * t14061;
    let t14065 = t3245 * t1750;
    let t14067 = t3209 * t3218;
    let t14068 = t4813 * t922;
    let t14069 = t14067 * t14068;
    let t14070 = t3200 * t14069;
    let t14072 = t9409 * t1022;
    let t14073 = t4818 * t922;
    (t14058, t14062, t14065, t14070, t14072, t14073)
}
