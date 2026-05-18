//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 979/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk979<F: Float>(t40731: F, t7720: F, t321: F, t8924: F, t262: F, t7204: F, t333: F, t7192: F, t1970: F, t236: F, t498: F, t5605: F, t7231: F) -> (F, F, F, F, F, F, F, F) {
    let t40732 = t7720 * t40731;
    let t40734 = t8924 * t321;
    let t40735 = t262 * t40734;
    let t40736 = t7204 * t40735;
    let t40738 = t8924 * t333;
    let t40739 = t262 * t40738;
    let t40740 = t7192 * t40739;
    let t40747 = t1970 * t7231 * t236 * t5605 * t498;
    (t40732, t40734, t40735, t40736, t40738, t40739, t40740, t40747)
}
