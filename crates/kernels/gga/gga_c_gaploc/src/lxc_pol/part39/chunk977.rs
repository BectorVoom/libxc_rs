//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 977/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk977<F: Float>(t2268: F, t2765: F, t9152: F, t39791: F, t39794: F, t39798: F, t12830: F, t29874: F, t39805: F, t39808: F, t39811: F, t12803: F, t1358: F) -> (F, F, F, F, F, F, F, F, F) {
    let t42814 = F::new(0.85365019907028448797e-1) * t2268 * t2765 * t9152;
    let t42815 = F::new(0.23712505529730124666e-2) * t39791;
    let t42816 = F::new(0.23712505529730124666e-2) * t39794;
    let t42817 = F::new(0.23712505529730124666e-2) * t39798;
    let t42820 = t29874 * t12830;
    let t42821 = F::new(0.71137516589190373998e-2) * t42820;
    let t42822 = F::new(0.16598753870811087267e-1) * t39805;
    let t42823 = F::new(0.23712505529730124666e-2) * t39808;
    let t42824 = F::new(0.23712505529730124666e-2) * t39811;
    let t42825 = t1358 * t12803;
    (t42814, t42815, t42816, t42817, t42821, t42822, t42823, t42824, t42825)
}
