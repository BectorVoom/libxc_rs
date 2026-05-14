//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 858/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk858<F: Float>(t7884: F, t8396: F, t2137: F, t32123: F, t1619: F, t322: F, t315: F, t309: F, t1219: F, t615: F, t525: F, t847: F, t448: F, t33428: F, t1432: F, t30147: F, t30148: F, t7842: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t33682 = t7884 * t8396;
    let t33698 = t2137 * t32123;
    let t33699 = t1619 * t322;
    let t33743 = t315 * t32123;
    let t33744 = t1619 * t309;
    let t33778 = t615 * t8396 * t1219;
    let t33787 = t525 * t847;
    let t33795 = t8396 * t448;
    let t33796 = t315 * t33795;
    let t33799 = t2137 * t33795;
    let t33802 = t315 * t33428;
    let t33831 = t30147 * t7842 * t30148 * t1432;
    (t33682, t33698, t33699, t33743, t33744, t33778, t33787, t33795, t33796, t33799, t33802, t33831)
}
