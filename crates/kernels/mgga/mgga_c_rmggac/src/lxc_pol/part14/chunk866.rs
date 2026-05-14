//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 866/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk866<F: Float>(t262: F, t40734: F, t7204: F, t333: F, t8924: F, t7192: F, t1970: F, t236: F, t498: F, t5605: F, t7231: F, t275: F, t9064: F, t4928: F, t645: F, t903: F) -> (F, F, F, F, F, F, F, F, F) {
    let t40735 = t262 * t40734;
    let t40736 = t7204 * t40735;
    let t40738 = t8924 * t333;
    let t40739 = t262 * t40738;
    let t40740 = t7192 * t40739;
    let t40747 = t1970 * t7231 * t236 * t5605 * t498;
    let t40750 = 2.0 * t275 * t9064;
    let t40756 = t645 * t4928;
    let t40757 = t903 * t40756;
    (t40735, t40736, t40738, t40739, t40740, t40747, t40750, t40756, t40757)
}
