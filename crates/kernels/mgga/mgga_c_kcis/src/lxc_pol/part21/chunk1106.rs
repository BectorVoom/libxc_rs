//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1106/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1106<F: Float>(t26857: F, t7699: F, t26742: F, t2173: F, t46978: F, t7710: F, t93661: F, t26714: F, t7687: F, t15573: F, t26735: F, t26717: F, t26728: F, t7690: F, t93609: F, t26823: F) -> (F, F, F, F, F, F, F, F, F) {
    let t93742 = t26857 * t7699;
    let t93750 = t26742 * t7699;
    let t93759 = t2173 * t46978 * t7710;
    let t93762 = t2173 * t93661;
    let t93764 = t7687 * t26714;
    let t93767 = t2173 * t15573 * t26735;
    let t93771 = t26728 * t26717;
    let t93773 = t7690 * t93609;
    let t93785 = t26823 * t7699;
    (t93742, t93750, t93759, t93762, t93764, t93767, t93771, t93773, t93785)
}
