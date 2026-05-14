//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 714/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk714<F: Float>(t44283: F, t123: F, t37975: F, t40165: F, t9074: F, t13401: F, t1358: F, t2299: F, t488: F, t42579: F, t3529: F, t874: F) -> (F, F, F, F, F) {
    let t44284 = 0.71137516589190373998e-2 * t44283;
    let t44285 = t37975 * t123;
    let t44287 = t9074 * t44285 * t40165;
    let t44288 = 0.142275033178380748e-1 * t44287;
    let t44292 = 0.31616674039640166221e-2 * t1358 * t2299 * t13401 * t488;
    let t44293 = 0.47425011059460249332e-2 * t42579;
    let t44294 = t3529 * t874;
    (t44284, t44288, t44292, t44293, t44294)
}
