//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 950/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk950<F: Float>(t448: F, t8396: F, t315: F, t7966: F, t2137: F, t7943: F, t33428: F, t2134: F, t119: F, t8993: F, t1432: F, t30147: F, t30148: F, t7842: F) -> (F, F, F, F, F, F, F, F) {
    let t33795 = t8396 * t448;
    let t33796 = t315 * t33795;
    let t33798 = F::new(0.17347256376410398924e1) * t33796 * t7966;
    let t33799 = t2137 * t33795;
    let t33801 = F::new(0.17347256376410398924e1) * t33799 * t7943;
    let t33802 = t315 * t33428;
    let t33804 = F::new(0.17347256376410398924e1) * t33802 * t2134;
    let t33818 = t119 * t8993;
    let t33831 = t30147 * t7842 * t30148 * t1432;
    (t33795, t33796, t33798, t33799, t33801, t33804, t33818, t33831)
}
