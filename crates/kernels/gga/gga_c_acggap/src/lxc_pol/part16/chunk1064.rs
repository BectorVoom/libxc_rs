//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1064/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1064<F: Float>(t1181: F, t30698: F, t38789: F, t599: F, t1479: F, t535: F, t7380: F, t4643: F, t8489: F, t2095: F, t1988: F, t9543: F) -> (F, F, F, F, F, F) {
    let t38792 = t30698 * t1181 * t599 * t38789;
    let t38795 = t535 * t1479;
    let t38796 = t7380 * t38795;
    let t38798 = t4643 * t8489;
    let t38799 = t2095 * t38798;
    let t38801 = t1988 * t9543;
    (t38792, t38795, t38796, t38798, t38799, t38801)
}
