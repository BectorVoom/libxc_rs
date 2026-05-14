//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1100/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1100<F: Float>(t19155: F, t11881: F, t1277: F, t1010: F, t11880: F, t502: F, t826: F, t2391: F, t263: F, t1289: F, t3358: F, t8395: F, t11036: F, t8370: F, t8373: F, t1070: F, t23353: F) -> (F, F, F, F, F, F, F, F) {
    let t40815 = t19155 * param_eta;
    let t40817 = t40815 * t11881 * t1277;
    let t40821 = t11880 * t502 * t1010 * t826;
    let t40822 = 4.0 * t40821;
    let t40825 = t11880 * t263 * t2391 * t826;
    let t40828 = t11880 * t11881 * t1289;
    let t40830 = t3358 * t8395;
    let t40833 = t11036 * t8370;
    let t40835 = t11036 * t8373;
    let t40837 = t23353 * t1070;
    (t40817, t40822, t40825, t40828, t40830, t40833, t40835, t40837)
}
