//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 726/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk726<F: Float>(t42587: F, t42590: F, t11182: F, t2317: F, t6525: F, t35900: F, t883: F, t2761: F, t9074: F, t1365: F, t36211: F, t35888: F, t35893: F, t4261: F, t11280: F, t2326: F) -> (F, F, F, F, F, F, F, F, F) {
    let t44305 = 0.142275033178380748e-1 * t42587;
    let t44306 = 0.142275033178380748e-1 * t42590;
    let t44308 = t6525 * t11182 * t2317;
    let t44309 = 0.11856252764865062333e-2 * t44308;
    let t44310 = t883 * t35900;
    let t44312 = t9074 * t2761 * t44310;
    let t44313 = 0.23712505529730124666e-2 * t44312;
    let t44315 = t6525 * t1365 * t36211;
    let t44316 = 0.11856252764865062333e-2 * t44315;
    let t44318 = t9074 * t1365 * t35888;
    let t44319 = 0.35568758294595186999e-2 * t44318;
    let t44321 = t9074 * t4261 * t35893;
    let t44322 = 0.23712505529730124666e-2 * t44321;
    let t44324 = t9074 * t11280 * t2326;
    (t44305, t44306, t44309, t44310, t44313, t44316, t44319, t44322, t44324)
}
