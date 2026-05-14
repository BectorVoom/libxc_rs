//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1184/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1184<F: Float>(t786: F, t8931: F, t2244: F, t3330: F, t1327: F, t6858: F, t260: F, t9022: F, t1340: F, t2245: F, t6859: F, t6982: F, t6966: F, t2323: F, t1352: F, t2284: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t26440 = t8931 * t786;
    let t26445 = t3330 * t2244;
    let t26450 = t1327 * t6858;
    let t26476 = t260 * t9022;
    let t26489 = t2245 * t1340;
    let t26492 = t6859 * t1340;
    let t26544 = t260 * t6982;
    let t26549 = t260 * t6966;
    let t26553 = t260 * t2323;
    let t26607 = t2284 * t1352;
    (t26440, t26445, t26450, t26476, t26489, t26492, t26544, t26549, t26553, t26607)
}
