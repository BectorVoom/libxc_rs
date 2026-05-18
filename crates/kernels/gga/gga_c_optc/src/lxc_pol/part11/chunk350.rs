//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 350/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk350<F: Float>(t1544: F, t155: F, t1150: F, t1159: F, t1162: F, t1170: F, t1177: F, t1179: F, t1520: F, t1529: F, t1533: F, t1536: F, t1541: F, t451: F, t459: F) -> (F, F) {
    let t1545 = t155 * t1544;
    let t1550 = F::new(0.11360101276506094136e1) * t1150 * t1529 - F::new(0.23181763972770020946e0) * t1533 * t459 + t1159 + F::new(0.28977204965962526182e-1) * t1162 * t1536 + F::new(0.5848048239485271795e1) * t1170 * t1541 - F::new(0.4030456356669135783e-1) * t1545 * t451 + t1177 + F::new(0.50380704458364197288e-2) * t1179 * t1520;
    (t1545, t1550)
}
