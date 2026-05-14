//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1177/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1177<F: Float>(t180: F, t8761: F, t1264: F, t6624: F, t2015: F, t3174: F, t676: F, t8630: F, t136: F, t2003: F, t3290: F, t1234: F, t6491: F, t24843: F, t3141: F, t8619: F) -> (F, F, F, F, F, F, F, F) {
    let t26016 = t180 * t8761;
    let t26029 = t6624 * t1264;
    let t26109 = t2015 * t3174;
    let t26111 = t676 * t8630;
    let t26114 = t136 * t2003 * t3290;
    let t26116 = t1234 * t6491;
    let t26129 = t24843 * t3141;
    let t26136 = t676 * t8619;
    (t26016, t26029, t26109, t26111, t26114, t26116, t26129, t26136)
}
