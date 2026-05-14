//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 353/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk353<F: Float>(t103: F, t1320: F, t566: F, t1323: F, t569: F, t568: F, t106: F, t9: F) -> (F, F, F, F) {
    let t1583 = t103 * t1320;
    let t1584 = t1583 * t566;
    let t1585 = t569 * t1323;
    let t1586 = t568 * t1585;
    let t1589 = t106 * t9;
    (t1583, t1584, t1586, t1589)
}
