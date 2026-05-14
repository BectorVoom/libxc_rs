//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1015/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1015<F: Float>(t11882: F, t23498: F, t263: F, t2928: F, t40815: F, t826: F, t11880: F, t11881: F, t2391: F, t37078: F, t40782: F, t40798: F, t40805: F, t40807: F, t41858: F, t41864: F, t42491: F, t42493: F, t42495: F, t42497: F, t42500: F, t42502: F, t42505: F, t42508: F) -> (F,) {
    let t42512 = t23498 * param_eta * t11882;
    let t42516 = t40815 * t263 * t2928 * t826;
    let t42519 = t11880 * t11881 * t2391;
    let t42521 = t42491 / 2.0 + t42493 / 2.0 - 3.0 / 4.0 * t42495 + t42497 / 4.0 + t42500 / 4.0 - 4.0 / 3.0 * t42502 + 2.0 * t42505 - 2.0 / 3.0 * t42508 - t41858 + t40782 + 22.0 / 9.0 * t37078 + t41864 - t40798 - t40805 - t40807 - 3.0 / 2.0 * t42512 + 3.0 * t42516 - 3.0 / 2.0 * t42519;
    (t42521,)
}
