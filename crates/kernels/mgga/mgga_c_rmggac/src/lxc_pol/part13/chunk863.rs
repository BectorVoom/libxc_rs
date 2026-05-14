//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 863/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk863<F: Float>(t40902: F, t41634: F, t40921: F, t8630: F, t36978: F, t40894: F, t40898: F, t7198: F, t16156: F, t9055: F, t2085: F, t8339: F, t1162: F, t1979: F, t1982: F, t201: F, t589: F) -> (F, F, F, F, F, F, F) {
    let t41635 = t41634 * t40902;
    let t41637 = t8630 * t40921;
    let t41639 = t36978 * t40894;
    let t41641 = t7198 * t40898;
    let t41654 = t16156 * t9055;
    let t41656 = t8339 * t2085;
    let t41663 = t589 * t1162 * t201 * t1979 * t1982;
    (t41635, t41637, t41639, t41641, t41654, t41656, t41663)
}
