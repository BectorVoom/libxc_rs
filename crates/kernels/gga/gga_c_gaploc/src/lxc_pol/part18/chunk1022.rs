//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1022/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1022<F: Float>(t11068: F, t590: F, t1991: F, t1628: F, t3495: F, t1589: F, t3451: F, t3464: F, t769: F, t10667: F, t314: F, t313: F) -> (F, F, F, F, F, F, F) {
    let t11069 = t11068 * t590;
    let t11071 = F::new(0.1022478025437886658e1) * t1991 * t11069;
    let t11072 = t1628 * t3495;
    let t11075 = t1589 * t3451;
    let t11080 = t769 * t3464;
    let t11083 = t314 * t10667;
    let t11084 = t313 * t11083;
    (t11069, t11071, t11072, t11075, t11080, t11083, t11084)
}
