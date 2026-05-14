//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 775/1135 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk775<F: Float>(t10243: F, t10244: F, t2531: F, t3239: F, t1936: F, t2493: F, t3243: F, t6182: F, t772: F, t10153: F, t268: F, t8508: F, t6853: F, t2210: F, t6857: F, t276: F) -> (F, F, F, F, F, F, F, F) {
    let t10245 = t10243 * t10244;
    let t10246 = t3239 * t2531;
    let t10247 = t10245 * t10246;
    let t10249 = t1936 * t2493;
    let t10250 = t3243 * t10249;
    let t10252 = t772 * t6182;
    let t10253 = t10153 * t10252;
    let t10255 = t8508 * t268;
    let t10256 = t10255 * t6853;
    let t10257 = t2210 * t6857;
    let t10258 = t10256 * t10257;
    let t10260 = t10243 * t276;
    (t10245, t10246, t10247, t10250, t10253, t10256, t10258, t10260)
}
