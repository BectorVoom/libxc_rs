//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 388/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk388<F: Float>(t1420: F, t385: F, t1388: F, t1394: F, t1397: F, t1401: F, t989: F, t992: F) -> (F, F) {
    let t1421 = t1420 * t385;
    let t1427 = 0.258925e1 * t1394 - t989 + 0.905775e0 * t1388 + 0.16504875e0 * t1397 - t992 + 0.248355e0 * t1401;
    (t1421, t1427)
}
