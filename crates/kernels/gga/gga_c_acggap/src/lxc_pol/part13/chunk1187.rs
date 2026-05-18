//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1187/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1187<F: Float>(t17912: F, t31443: F, t33953: F, t5207: F, t142: F, t5160: F, t7436: F, t2030: F, t4495: F, t7815: F, t2060: F, t5187: F) -> (F, F, F, F) {
    let t36250 = t31443 * t17912 * t33953 * t5207;
    let t36253 = t7436 * t142 * t5160;
    let t36256 = t2030 * t7815 * t4495;
    let t36259 = t2060 * t7815 * t5187;
    (t36250, t36253, t36256, t36259)
}
