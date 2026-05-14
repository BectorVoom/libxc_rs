//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 760/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk760<F: Float>(t3171: F, t820: F, t3443: F, t869: F, t896: F, t3444: F, t315: F, t9184: F, t277: F, t9179: F, t2438: F, t3439: F, t325: F, t8769: F, t2639: F, t1936: F, t2520: F) -> (F, F, F, F, F, F, F, F) {
    let t9747 = t3171 * t820;
    let t9748 = t3443 * t9747;
    let t9750 = t869 * t896;
    let t9751 = t9750 * t3444;
    let t9753 = t9184 * t315;
    let t9754 = t3443 * t9753;
    let t9756 = t277 * t9179;
    let t9757 = t2438 * t3439;
    let t9758 = t9756 * t9757;
    let t9760 = t325 * t8769;
    let t9761 = t9760 * t2639;
    let t9763 = t2520 * t1936;
    (t9748, t9751, t9754, t9756, t9758, t9760, t9761, t9763)
}
