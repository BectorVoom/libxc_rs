//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 760/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk760<F: Float>(t12800: F, t6305: F, t2268: F, t2792: F, t3158: F, t12810: F, t6313: F, t2756: F, t3148: F, t12834: F, t12826: F, t26938: F, t3133: F, t31591: F, t4261: F, t9074: F) -> (F, F, F, F, F, F, F, F, F) {
    let t42691 = 0.19918504644973304719e0 * t6305 * t12800;
    let t42694 = 0.19918504644973304719e0 * t2268 * t3158 * t2792;
    let t42698 = 0.37940008847568199465e-1 * t6313 * t12810;
    let t42703 = 0.28455006635676149599e-1 * t6305 * t12810;
    let t42706 = 0.28455006635676149599e-1 * t2268 * t3148 * t2756;
    let t42708 = 0.37940008847568199465e-1 * t6313 * t12834;
    let t42712 = 0.34146007962811379518e0 * t6305 * t12826;
    let t42715 = 0.34146007962811379518e0 * t2268 * t26938 * t3133;
    let t42717 = t9074 * t4261 * t31591;
    (t42691, t42694, t42698, t42703, t42706, t42708, t42712, t42715, t42717)
}
