//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1011/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1011<F: Float>(t40806: F, t40821: F, t40840: F, t40844: F, t12241: F, t833: F, t1299: F, t3730: F, t12351: F, t1348: F, t3774: F, t6767: F, t1338: F, t6755: F, t39244: F, t39251: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t41871 = 8.0 / 3.0 * t40806;
    let t41877 = 8.0 * t40821;
    let t41885 = 4.0 / 3.0 * t40840;
    let t41887 = 4.0 / 3.0 * t40844;
    let t41901 = t12241 * t833;
    let t41906 = t3730 * t1299;
    let t42101 = t1348 * t12351;
    let t42106 = t6767 * t3774;
    let t42121 = t1338 * t12351;
    let t42128 = t6755 * t3774;
    let t42162 = 0.1440846329149835838e-2 * t39244;
    let t42164 = 0.1440846329149835838e-2 * t39251;
    (t41871, t41877, t41885, t41887, t41901, t41906, t42101, t42106, t42121, t42128, t42162, t42164)
}
