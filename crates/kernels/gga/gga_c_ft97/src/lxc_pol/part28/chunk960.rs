//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 960/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk960<F: Float>(t1058: F, t7339: F, t1984: F, t34918: F, t1349: F, t138445: F, t138476: F, t138511: F, t1969: F, t23925: F, t26800: F, t26809: F, t26811: F, t26817: F, t28: F, t3051: F, t3052: F, t32717: F, t32719: F, t32722: F, t32879: F, t32881: F, t3450: F, t35010: F, t379: F, t5772: F, t5779: F, t7308: F, t9073: F, t925: F, t9432: F) -> (F, F) {
    let t147112 = t7339 * t1058;
    let t147122 = t1984 * t34918;
    let t147132 = -t5772 * t1969 * t138511 * t925 / 18.0 + 2.0 / 9.0 * t26809 * t9073 * t32717 * t3052 - t5772 * t138445 * t26800 / 3.0 - t7308 * t3051 * t26811 / 9.0 + t26817 * t32719 / 9.0 - 2.0 / 9.0 * t26809 * t1969 * t32879 * t3052 - t5772 * t1969 * t147112 * t379 / 18.0 - t26817 * t32881 / 9.0 + t5772 * t9432 * t32722 * t3450 - t1349 * t28 * t147122 * t5779 / 3.0 - 2.0 / 3.0 * t1349 * t28 * t23925 * t35010 - t138476 / 18.0;
    (t147122, t147132)
}
