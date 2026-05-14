//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 881/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk881<F: Float>(t2099: F, t3886: F, t2367: F, t2382: F, t3919: F, t2381: F, t2411: F, t3730: F, t824: F, t758: F, t1235: F, t297: F, t46: F, t3232: F) -> (F, F, F, F, F, F, F, F) {
    let t10241 = t2099 * t3886;
    let t10242 = t2367 * t10241;
    let t10244 = t3919 * t2382;
    let t10245 = t2381 * t10244;
    let t10251 = t2411 * t3730;
    let t10252 = t10251 * t824;
    let t10253 = t758 * t10252;
    let t10257 = t1235 * t297 * t46;
    let t10258 = t3232 * t10257;
    (t10241, t10242, t10244, t10245, t10251, t10252, t10253, t10258)
}
