//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 997/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk997<F: Float>(t18587: F, t338: F, t2241: F, t18439: F, t6198: F, t828: F, t2195: F, t2238: F, t2411: F, t54: F, t300: F, t6404: F, t2255: F, t2277: F, t356: F, t18442: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t18589 = t338 / t18587;
    let t18591 = t2241 * t2241;
    let t18592 = 1.0 / t18591;
    let t18596 = 0.96141975308641975307e-1 * t18439;
    let t18612 = t828 * t6198;
    let t18617 = t338 / t2238 / t2195;
    let t18657 = t54 * t2411;
    let t18661 = t300 * t6404;
    let t18706 = t356 / t2277 / t2255;
    let t18750 = 0.17757530864197530864e0 * t18439;
    let t18765 = 0.5356037037037037037e1 * t18439;
    let t18766 = 0.16979925925925925926e1 * t18442;
    (t18589, t18592, t18596, t18612, t18617, t18657, t18661, t18706, t18750, t18765, t18766)
}
