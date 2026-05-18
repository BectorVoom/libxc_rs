//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1068/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1068<F: Float>(t5558: F, t952: F, t1959: F, t2590: F, t119: F, t19077: F, t481: F, t19223: F, t19244: F, t1570: F, t21488: F, t565: F) -> (F, F, F, F, F, F) {
    let t23555 = t952 * t5558;
    let t23575 = t2590 * t1959;
    let t23609 = t481 * t19077 * t119;
    let t23726 = t481 * t19223 * t119;
    let t23741 = t481 * t19244 * t119;
    let t23759 = t21488 * t565 * t1570;
    (t23555, t23575, t23609, t23726, t23741, t23759)
}
