//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 358/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk358<F: Float>(t1630: F, t1629: F, t116: F, t128: F, t195: F, t144: F, t599: F, t122: F, t125: F, t1457: F, t169: F) -> (F, F, F, F, F, F) {
    let t1631 = F::new(1.0) / t1630;
    let t1632 = t1629 * t1631;
    let t1633 = t116 * t1632;
    let t1636 = t128 * t195;
    let t1638 = t1636 * t144 * t599;
    let t1642 = t1457 * t122 * t125;
    let t1643 = t169 * t1642;
    (t1631, t1633, t1636, t1638, t1642, t1643)
}
