//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 981/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk981<F: Float>(t5697: F, t9962: F, t5701: F, t5608: F, t5675: F, t9934: F, t2661: F, t2482: F, t4000: F, t814: F, t136: F, t550: F, t220: F, t124: F, t1882: F, t5609: F, t9794: F) -> (F, F, F, F, F, F, F) {
    let t13810 = t9962 * t5697;
    let t13813 = 0.20007875121765877254e-2 * t9962 * t5701;
    let t13829 = t5608 * t5675;
    let t13830 = t9934 * t13829;
    let t13832 = 0.28582678745379824648e-4 * t2661 * t13830;
    let t13845 = t2482 * t4000 * t814;
    let t13846 = t550 * t136;
    let t13847 = t13846 * t220;
    let t13848 = t124 * t1882;
    let t13850 = t13847 * t13848 * t5675;
    let t13851 = t13845 * t13850;
    let t13857 = t9794 * t5609;
    (t13810, t13813, t13832, t13847, t13848, t13851, t13857)
}
