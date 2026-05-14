//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1370/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1370<F: Float>(t27326: F, t8392: F, t160: F, t26768: F, t605: F, t9016: F, t27208: F, t27212: F, t105444: F, t11593: F, t12719: F, t12987: F, t12991: F, t13022: F, t13208: F, t1901: F, t2190: F, t2221: F, t23443: F, t26863: F, t27007: F, t27068: F, t3425: F, t379: F, t50550: F, t5916: F, t6630: F, t9144: F, t9419: F, t95651: F, t95653: F, t95696: F) -> (F,) {
    let t106708 = 2.0 / 27.0 * t8392 * t27326;
    let t106724 = t160 * t26768;
    let t106729 = t9016 * t605;
    let t106745 = 2.0 / 27.0 * t8392 * t27208;
    let t106747 = 4.0 / 27.0 * t8392 * t27212;
    let t106748 = -2.0 / 9.0 * t95651 - 2.0 / 9.0 * t95653 - t106708 - 2.0 / 27.0 * t1901 * t26863 * t12719 + 2.0 / 9.0 * t1901 * t95696 * t3425 + t1901 * t23443 * t13022 / 9.0 + 4.0 / 9.0 * t11593 * t23443 * t12987 + 2.0 / 9.0 * t1901 * t9419 * t27007 + 2.0 / 9.0 * t1901 * t2221 * t106724 * t379 + 4.0 * t1901 * t106729 * t6630 * t2190 - 4.0 / 9.0 * t11593 * t9144 * t5916 * t12991 - 4.0 / 9.0 * t1901 * t50550 * t27068 - 4.0 / 9.0 * t1901 * t13208 * t105444 + t106745 + t106747;
    (t106748,)
}
