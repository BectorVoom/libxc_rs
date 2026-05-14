//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 958/1209 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk958<F: Float>(t1037: F, t1303: F, t4048: F, t6: F, t1153: F, t1418: F, t122: F, t169: F, t188: F, t4054: F, t4687: F, t5407: F, t505: F, t681: F, t5199: F, t5214: F, t5217: F) -> (F, F, F, F, F, F, F, F) {
    let t21183 = t1037 * t1303;
    let t21204 = t4048 * t6;
    let t21249 = t1418 * t1153;
    let t21281 = t169 * t4048 * t122 * t188;
    let t21283 = t4054 * t6;
    let t21369 = t5407 * t4687;
    let t21625 = t681 * t505;
    let t21631 = t5214 * t5199 * t5217;
    (t21183, t21204, t21249, t21281, t21283, t21369, t21625, t21631)
}
