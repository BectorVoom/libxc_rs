//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1004/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1004<F: Float>(t12228: F, t12081: F, t12084: F, t12087: F, t12092: F, t12095: F, t12100: F, t12103: F, t12109: F, t12111: F, t12200: F, t12204: F, t12207: F, t12211: F, t12213: F, t12216: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t41132 = 2.0 * t12228;
    let t41133 = 3.0 / 2.0 * t12081;
    let t41134 = t12084 / 2.0;
    let t41135 = t12087 / 2.0;
    let t41138 = t12092 / 2.0;
    let t41139 = 15.0 / 8.0 * t12095;
    let t41140 = 5.0 / 8.0 * t12100;
    let t41141 = 5.0 / 8.0 * t12103;
    let t41142 = 3.0 / 2.0 * t12109;
    let t41143 = t12111 / 2.0;
    let t41144 = t12200 / 2.0;
    let t41145 = 5.0 / 8.0 * t12204;
    let t41147 = 3.0 / 2.0 * t12207;
    let t41148 = 3.0 / 2.0 * t12211;
    let t41149 = 3.0 / 2.0 * t12213;
    let t41150 = 3.0 * t12216;
    (t41132, t41133, t41134, t41135, t41138, t41139, t41140, t41141, t41142, t41143, t41144, t41145, t41147, t41148, t41149, t41150)
}
