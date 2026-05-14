//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 826/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk826<F: Float>(t1993: F, t1997: F, t6461: F, t1828: F, t549: F, t19: F, t1966: F, t1970: F, t2029: F, t676: F, t2003: F, t764: F, t136: F, t2164: F, t168: F, t692: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6463 = t1993 * t6461 * t1997;
    let t6465 = t549 * t1828;
    let t6466 = t19 * t6465;
    let t6478 = t549 * t1966;
    let t6479 = t19 * t6478;
    let t6481 = t549 * t1970;
    let t6482 = t19 * t6481;
    let t6487 = t676 * t2029;
    let t6491 = t2003 * t764;
    let t6492 = t136 * t6491;
    let t6494 = t549 * t2164;
    let t6495 = t136 * t6494;
    let t6507 = 1.0 / t168 / t692;
    (t6463, t6465, t6466, t6478, t6479, t6481, t6482, t6487, t6491, t6492, t6494, t6495, t6507)
}
