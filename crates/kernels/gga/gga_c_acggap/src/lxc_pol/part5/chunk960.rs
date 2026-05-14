//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 960/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk960<F: Float>(t1016: F, t1410: F, t1451: F, t3228: F, t1005: F, t4503: F, t1603: F, t323: F, t851: F, t315: F, t5299: F, t3101: F, t316: F, t449: F, t556: F, t322: F, t5331: F) -> (F, F, F, F, F, F, F) {
    let t18834 = t1016 * t1410;
    let t18839 = t3228 * t1451;
    let t18841 = t1005 * t4503;
    let t18858 = t851 * t1603 * t323;
    let t18861 = t315 * t5299 * t323;
    let t18866 = t316 * t449 * t556 * t3101;
    let t18872 = t316 * t449 * t5331 * t322;
    (t18834, t18839, t18841, t18858, t18861, t18866, t18872)
}
