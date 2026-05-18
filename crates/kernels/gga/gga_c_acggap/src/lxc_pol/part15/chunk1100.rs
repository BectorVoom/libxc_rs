//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1100/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1100<F: Float>(t31428: F, t9614: F, t2030: F, t20559: F, t8923: F, t1016: F, t2060: F, t507: F, t8928: F, t301: F, t4256: F, t7450: F, t9536: F) -> (F, F, F, F) {
    let t39002 = t31428 * t9614;
    let t39005 = t2030 * t20559 * t8923;
    let t39009 = t2060 * t507 * t1016 * t8928;
    let t39013 = t7450 * t4256 * t9536 * t301;
    (t39002, t39005, t39009, t39013)
}
