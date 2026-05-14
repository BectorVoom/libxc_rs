//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1199/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1199<F: Float>(t10146: F, t2952: F, t9851: F, t10181: F, t1157: F, t1126: F, t9849: F, t653: F, t7853: F, t7831: F, t10064: F, t516: F, t10165: F, t3728: F, t518: F, t9988: F, tau1: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t28042 = t2952 * t10146;
    let t28043 = t9851 * tau1;
    let t28048 = t1157 * t10181;
    let t28053 = t1126 * t9849;
    let t28056 = t7853 * t653;
    let t28057 = t1157 * t28056;
    let t28061 = t7831 * t653;
    let t28062 = t1126 * t28061;
    let t28067 = t516 * t10064 * t653;
    let t28070 = t3728 * t10165;
    let t28087 = t9988 * t518;
    (t28042, t28043, t28048, t28053, t28056, t28057, t28061, t28062, t28067, t28070, t28087)
}
