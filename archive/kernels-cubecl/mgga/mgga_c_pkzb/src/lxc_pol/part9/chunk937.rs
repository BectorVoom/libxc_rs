//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 937/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk937<F: Float>(t1107: F, t5490: F, t1956: F, t5493: F, t730: F, t2816: F, t702: F, t1096: F, t1932: F, t1917: F, t2819: F, t1940: F, t2815: F) -> (F, F, F, F, F, F, F, F) {
    let t7226 = t5490 * t1107;
    let t7227 = t5493 * t1956;
    let t7228 = t7226 * t7227;
    let t7230 = F::cast_from(0.10254018858216406658e4_f64) * t730 * t7228;
    let t7231 = t2816 * t702;
    let t7234 = t1096 * t1932;
    let t7237 = t2819 * t1917;
    let t7240 = t2815 * t1940;
    (t7226, t7227, t7228, t7230, t7231, t7234, t7237, t7240)
}
