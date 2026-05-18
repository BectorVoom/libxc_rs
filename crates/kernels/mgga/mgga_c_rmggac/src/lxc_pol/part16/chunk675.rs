//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 675/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk675<F: Float>(t118: F, t9781: F, t1986: F, t1994: F, t2283: F, t2412: F, t128: F, t1910: F, t2001: F, t675: F, t2286: F, t1934: F) -> (F, F, F, F, F, F, F) {
    let t9782 = t118 * t9781;
    let t9783 = t1986 * t9782;
    let t9784 = t1994 * t9783;
    let t9786 = t2412 * t2283;
    let t9788 = t128 * t1910;
    let t9789 = t118 * t9788;
    let t9790 = t2001 * t9789;
    let t9791 = t675 * t9790;
    let t9793 = t2412 * t2286;
    let t9795 = t1986 * t1934;
    (t9783, t9784, t9786, t9790, t9791, t9793, t9795)
}
