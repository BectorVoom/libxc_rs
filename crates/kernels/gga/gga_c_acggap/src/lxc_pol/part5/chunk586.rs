//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 586/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk586<F: Float>(t177: F, t377: F, t973: F, t1963: F, t22: F, t161: F, t151: F, t415: F, t968: F, t1077: F, t145: F, t334: F, t986: F) -> (F, F, F, F, F, F, F) {
    let t3556 = F::new(0.17006693853500995666e-1) * t377 * t973 * t177;
    let t3558 = F::new(1.0) / t22 / t1963;
    let t3559 = t161 * t3558;
    let t3562 = F::new(0.37792653007779990369e-1) * t151 * t3559 * t177;
    let t3563 = t415 * t968;
    let t3565 = t1077 * t145;
    let t3570 = t986 * t334;
    (t3556, t3558, t3559, t3562, t3563, t3565, t3570)
}
