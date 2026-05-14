//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 749/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk749<F: Float>(t12823: F, t15737: F, t15746: F, t3499: F, t16732: F, t2102: F, t16682: F, t1775: F, t4762: F, t16687: F, t9217: F, t16694: F, t15768: F, t3506: F, t15763: F, t16712: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t17296 = t12823 * t15737;
    let t17299 = t3499 * t15746;
    let t17302 = t2102 * t16732;
    let t17305 = t2102 * t16682;
    let t17310 = t1775 * t4762;
    let t17313 = t9217 * t16687;
    let t17316 = t2102 * t16694;
    let t17319 = t3506 * t15768;
    let t17322 = t3499 * t15763;
    let t17325 = t2102 * t16712;
    (t17296, t17299, t17302, t17305, t17310, t17313, t17316, t17319, t17322, t17325)
}
