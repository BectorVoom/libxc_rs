//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 469/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk469<F: Float>(t334: F, t3688: F, t1197: F, t45: F, t1202: F, t330: F, t1210: F) -> (F, F, F, F) {
    let t3689 = t3688 * t334;
    let t3692 = t45 * t1197;
    let t3695 = t1202 * t330;
    let t3696 = F::new(1.0) / t3695;
    let t3697 = t1210 * t1210;
    (t3689, t3692, t3696, t3697)
}
