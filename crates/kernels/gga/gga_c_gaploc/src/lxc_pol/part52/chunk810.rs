//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 810/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk810<F: Float>(t40165: F, t44285: F, t9074: F, t13401: F, t1358: F, t2299: F, t488: F, t42579: F, t3529: F, t874: F) -> (F, F, F, F) {
    let t44287 = t9074 * t44285 * t40165;
    let t44288 = F::new(0.142275033178380748e-1) * t44287;
    let t44292 = F::new(0.31616674039640166221e-2) * t1358 * t2299 * t13401 * t488;
    let t44293 = F::new(0.47425011059460249332e-2) * t42579;
    let t44294 = t3529 * t874;
    (t44288, t44292, t44293, t44294)
}
