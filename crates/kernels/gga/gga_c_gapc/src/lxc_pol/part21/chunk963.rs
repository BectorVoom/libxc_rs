//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 963/1125 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk963<F: Float>(t11923: F, t11927: F, t3363: F, t1461: F, t8710: F, t1084: F, t28517: F, t26662: F, t640: F, t16798: F, t7451: F, t15548: F, t7073: F, t21801: F, t2660: F, t7330: F) -> (F, F, F, F, F, F, F, F) {
    let t33617 = t3363 * t11923 * t11927;
    let t33619 = t1461 * t8710;
    let t33620 = t1084 * t33619;
    let t33621 = t33620 * t28517;
    let t33623 = t640 * t26662;
    let t33625 = t7451 * t33623 * t16798;
    let t33628 = t7073 * t33623 * t15548;
    let t33631 = t2660 * t21801 * t7330;
    (t33617, t33619, t33620, t33621, t33623, t33625, t33628, t33631)
}
