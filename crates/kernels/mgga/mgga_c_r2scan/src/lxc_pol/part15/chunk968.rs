//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 968/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk968<F: Float>(t37822: F, t1415: F, t2116: F, t5: F, t511: F, t57: F, t7: F, t2158: F, t37699: F, t10844: F, t10899: F, t2201: F, t10848: F, t2207: F, t10894: F, t1628: F) -> (F, F, F, F, F, F) {
    let t37823 = 0.71120679974571020322e0 * t37822;
    let t37833 = t5 * t7 * t1415 * t511 * t57 * t2116;
    let t37834 = 0.89443204944342177673e-3 * t37833;
    let t37835 = t37699 * t2158;
    let t37838 = t2201 * t10899 * t10844;
    let t37841 = t2207 * t10899 * t10848;
    let t37843 = t10894 * t1628;
    (t37823, t37834, t37835, t37838, t37841, t37843)
}
