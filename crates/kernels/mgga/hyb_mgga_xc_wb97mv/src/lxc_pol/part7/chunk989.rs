//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 989/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk989<F: Float>(t9625: F, t9641: F, t9650: F, t9705: F, t540: F, t1169: F, t3673: F, t1512: F, t2961: F, t1558: F, t2813: F, t1104: F, t3842: F, t2922: F, t3799: F, t1522: F, t2839: F) -> (F, F, F, F, F, F, F, F) {
    let t9707 = t9625 + t9641 + t9650 + t9705;
    let t9708 = t9707 * t540;
    let t9709 = t3673 * t1169;
    let t9711 = t1512 * t2961;
    let t9712 = t2813 * t1558;
    let t9713 = t1104 * t3842;
    let t9715 = t2922 * t3799;
    let t9718 = t1522 * t2839;
    (t9707, t9708, t9709, t9711, t9712, t9713, t9715, t9718)
}
