//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 736/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk736<F: Float>(t2325: F, t36117: F, t882: F, t883: F, t13273: F, t2312: F, t42820: F, t13258: F, t2321: F, t38051: F, t9074: F, t1063: F, t3565: F, t6750: F, t2268: F, t2765: F, t34267: F) -> (F, F, F, F, F, F, F) {
    let t44512 = t882 * t2325 * t883 * t36117;
    let t44513 = 0.11856252764865062333e-2 * t44512;
    let t44514 = t2312 * t13273;
    let t44515 = 0.23712505529730124666e-2 * t44514;
    let t44516 = 0.142275033178380748e-1 * t42820;
    let t44517 = t2312 * t13258;
    let t44518 = 0.11856252764865062333e-2 * t44517;
    let t44520 = t9074 * t38051 * t2321;
    let t44521 = 0.11856252764865062333e-2 * t44520;
    let t44524 = 0.19918504644973304719e0 * t1063 * t3565 * t6750;
    let t44527 = 0.39837009289946609438e0 * t2268 * t2765 * t34267;
    (t44513, t44515, t44516, t44518, t44521, t44524, t44527)
}
