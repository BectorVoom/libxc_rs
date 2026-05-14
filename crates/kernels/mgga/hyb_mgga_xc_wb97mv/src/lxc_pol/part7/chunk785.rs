//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 785/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk785<F: Float>(t2541: F, t2546: F, t3478: F, t3520: F, t4285: F, t4297: F, t4301: F, t4305: F, t4307: F, t4312: F, t4316: F, t976: F) -> (F, F) {
    let t4345 = -0.17648625e1 * t4297 + 0.3529725e1 * t4301 + t2541 - 0.103295e1 * t3478 + 0.1549425e1 * t4285 + 0.31558125e0 * t4305 + 0.6311625e0 * t4307 + t2546 - 0.41678e0 * t3520 + 0.312585e0 * t4312 + 0.312585e0 * t4316;
    let t4346 = t4345 * t976;
    (t4345, t4346)
}
