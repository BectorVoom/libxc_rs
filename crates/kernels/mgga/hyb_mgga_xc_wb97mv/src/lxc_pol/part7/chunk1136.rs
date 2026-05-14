//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1136/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1136<F: Float>(t2589: F, t7318: F, t2593: F, t2596: F, t2571: F, t2224: F, t238: F, t2503: F, t2507: F, t2064: F, t339: F, t22470: F, t351: F, t6812: F, t952: F, t21425: F, t222: F, t341: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t23058 = t7318 * t2589;
    let t23077 = t2593 * t2593;
    let t23078 = 1.0 / t23077;
    let t23081 = t2596 * t2596;
    let t23082 = 1.0 / t23081;
    let t23116 = 1.0 / t2593 / t2571;
    let t23132 = t238 * t2224 * t2503;
    let t23135 = t238 * t2224 * t2507;
    let t23152 = 1.0 / t339 / t2064;
    let t23171 = t238 * t22470 * t351;
    let t23172 = 0.13490888888888888889e1 * t23171;
    let t23174 = t238 * t6812 * t952;
    let t23177 = t222 * t21425 * t341;
    (t23058, t23078, t23082, t23116, t23132, t23135, t23152, t23171, t23172, t23174, t23177)
}
