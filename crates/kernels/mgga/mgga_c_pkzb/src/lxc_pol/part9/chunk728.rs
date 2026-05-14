//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 728/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk728<F: Float>(t1719: F, t1721: F, t179: F, t568: F, t600: F, t1753: F, t173: F, t607: F, t614: F, t1730: F) -> (F, F, F, F, F, F, F) {
    let t5245 = t1719 * t1721;
    let t5247 = t179 * t5245 * t568;
    let t5250 = t600 * t1721;
    let t5251 = t5250 * t1753;
    let t5252 = t179 * t5251;
    let t5255 = t607 * t173;
    let t5256 = t5255 * t614;
    let t5257 = t1730 * t5256;
    (t5247, t5250, t5251, t5252, t5255, t5256, t5257)
}
