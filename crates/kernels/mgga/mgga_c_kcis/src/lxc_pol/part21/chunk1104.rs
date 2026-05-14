//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1104/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1104<F: Float>(t26854: F, t7687: F, t15573: F, t26731: F, t2173: F, t10995: F, t2836: F, t93157: F, t26783: F, t26781: F, t26717: F, t2865: F, t979: F, t990: F, t46978: F, t7692: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t93606 = t7687 * t26854;
    let t93609 = t15573 * t26731;
    let t93610 = t2173 * t93609;
    let t93620 = t2836 * t10995;
    let t93628 = 0.73697530864197530862e-3 * t93157;
    let t93637 = t15573 * t26783;
    let t93638 = t26781 * t93637;
    let t93653 = t7687 * t26717;
    let t93658 = t979 * t2865 * t990;
    let t93661 = t46978 * t7692;
    (t93606, t93609, t93610, t93620, t93628, t93637, t93638, t93653, t93658, t93661)
}
