//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1305/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1305<F: Float>(t23205: F, t2481: F, t4296: F, t4300: F, t7266: F, t23217: F, t7282: F, t11256: F, t2486: F, t3502: F, t9367: F, t11283: F, t2480: F, t941: F, t11261: F, t11290: F) -> (F, F, F, F, F, F, F, F, F) {
    let t31880 = t23205 * t4296 * t2481;
    let t31883 = t7266 * t4300 * t2481;
    let t31886 = t23217 * t4296 * t2481;
    let t31889 = t7282 * t4300 * t2481;
    let t31891 = t11256 * t2486;
    let t31893 = t3502 * t9367;
    let t31896 = t2480 * t11283 * t941;
    let t31898 = t11261 * t2486;
    let t31900 = t11290 * t2486;
    (t31880, t31883, t31886, t31889, t31891, t31893, t31896, t31898, t31900)
}
