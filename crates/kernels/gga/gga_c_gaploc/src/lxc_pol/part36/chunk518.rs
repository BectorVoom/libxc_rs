//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 518/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk518<F: Float>(t2464: F, t9850: F, t825: F, t313: F, t9725: F, t9739: F, t1645: F, t9740: F, t2530: F, t2667: F, t1445: F, t2033: F, t2087: F, t2639: F, t3277: F, t6111: F, t9832: F, t9836: F, t9838: F, t9839: F, t9842: F, t9846: F, t9849: F) -> (F, F, F) {
    let t9851 = t2464 * t9850;
    let t9852 = t825 * t9851;
    let t9853 = 0.85206502119823888169e-1 * t9852;
    let t9854 = t313 * t9725;
    let t9857 = t313 * t9739;
    let t9858 = t1645 * t9740;
    let t9863 = t2667 * t2530;
    let t9864 = t1445 * t9863;
    let t9867 = t9832 - t9836 + t9838 - 0.79445533226334281487e-1 * t6111 * t9839 + 0.39722766613167140743e-1 * t2033 * t9842 - t9846 - t9849 + t9853 - 0.10725146985555128001e1 * t9854 * t2639 + 0.42900587942220512003e1 * t9857 * t9858 - 0.25025342966295298669e1 * t3277 * t2639 - 0.13803453343411469884e2 * t2087 * t9864;
    (t9853, t9858, t9867)
}
