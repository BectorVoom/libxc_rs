//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1098/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1098<F: Float>(t1403: F, t1689: F, t1509: F, t5685: F, t1037: F, t1303: F, t4048: F, t6: F, t1153: F, t1418: F, t122: F, t169: F, t188: F) -> (F, F, F, F, F, F) {
    let t21115 = t1689 * t1403;
    let t21157 = t5685 * t1509;
    let t21183 = t1037 * t1303;
    let t21204 = t4048 * t6;
    let t21249 = t1418 * t1153;
    let t21281 = t169 * t4048 * t122 * t188;
    (t21115, t21157, t21183, t21204, t21249, t21281)
}
