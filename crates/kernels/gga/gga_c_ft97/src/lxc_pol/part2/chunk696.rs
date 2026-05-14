//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 696/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk696<F: Float>(t12302: F, t1969: F, t3281: F, t1882: F, t3324: F, t3327: F, t3320: F, t1017: F, t1570: F, t1559: F, t446: F, t1986: F, t925: F, t9073: F, t11050: F, t569: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12303 = t1969 * t12302;
    let t12304 = t3281 * t12303;
    let t12306 = t1882 * t3324;
    let t12307 = t12306 / 27.0;
    let t12308 = t1882 * t3327;
    let t12309 = 2.0 / 27.0 * t12308;
    let t12310 = t1882 * t3320;
    let t12311 = 2.0 / 81.0 * t12310;
    let t12312 = t1017 * t1570;
    let t12313 = t12312 * t1559;
    let t12314 = t1969 * t12313;
    let t12315 = t446 * t12314;
    let t12317 = t925 * t1986;
    let t12318 = t9073 * t12317;
    let t12319 = t446 * t12318;
    let t12321 = t569 * t11050;
    (t12304, t12306, t12307, t12308, t12309, t12310, t12311, t12313, t12315, t12317, t12319, t12321)
}
