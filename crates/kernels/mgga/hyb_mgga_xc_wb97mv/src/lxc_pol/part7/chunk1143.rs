//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1143/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1143<F: Float>(t2626: F, t2647: F, t2685: F, t2694: F, t1014: F, t7706: F, t7737: F, t7740: F, t7755: F, t2690: F) -> (F, F, F, F, F, F) {
    let t23570 = 0.43374325201206959368e-1 * t2626 * t2647 * t2685;
    let t23574 = 0.12842595503380418954e1 * t2626 * t2647 * t2694;
    let t23577 = 0.38527786510141256862e1 * t2626 * t1014 * t7706;
    let t23578 = t7737 * t7740;
    let t23582 = 0.1301229756036208781e0 * t2626 * t1014 * t7755;
    let t23583 = t2690 * t2690;
    (t23570, t23574, t23577, t23578, t23582, t23583)
}
