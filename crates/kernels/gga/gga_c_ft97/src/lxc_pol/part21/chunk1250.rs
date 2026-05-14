//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1250/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1250<F: Float>(t1349: F, t30111: F, t376: F, t104265: F, t104379: F, t17099: F, t17189: F, t1969: F, t23413: F, t24080: F, t24081: F, t26535: F, t26540: F, t26567: F, t26809: F, t28: F, t30141: F, t30156: F, t30284: F, t3052: F, t4714: F, t4822: F, t5772: F, t5773: F, t5778: F, t5781: F, t5849: F, t614: F, t6580: F, t94230: F, t9432: F) -> (F,) {
    let t119221 = t1349 * t376 * t30111;
    let t119245 = -2.0 / 9.0 * t26809 * t1969 * t26567 * t3052 - t23413 * t30141 / 9.0 - t1349 * t28 * t5778 * t614 * t4714 / 3.0 + 2.0 / 9.0 * t119221 - t30156 * t5781 / 3.0 - 2.0 / 3.0 * t6580 * t26535 - 2.0 / 3.0 * t6580 * t26540 - t104379 + t5772 * t9432 * t5773 * t17099 + t30156 * t5849 / 6.0 + 2.0 / 9.0 * t5772 * t94230 * t30284 + 2.0 / 9.0 * t5772 * t24080 * t104265 * t4822 + 2.0 / 9.0 * t5772 * t24080 * t24081 * t17189;
    (t119245,)
}
