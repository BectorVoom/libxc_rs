//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1041/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1041<F: Float>(t35258: F, t35271: F, t31029: F, t31033: F, t31039: F, t31041: F, t31045: F, t31049: F, t31060: F, t31074: F, t31081: F, t31083: F, t31095: F, t31100: F, t32677: F, t35260: F, t35264: F, t35273: F) -> (F,) {
    let t37458 = 0.32012600194825403606e-1 * t35258;
    let t37464 = 0.21437009059034868486e-3 * t35271;
    let t37471 = 0.4584375e-1 * t31029 + 0.916875e-1 * t31033 + t32677 + 0.16006300097412701803e-1 * t31039 - 0.85748036236139473944e-3 * t31041 + t37458 - 0.75475421495049964966e-2 * t35260 + 0.62896184579208304138e-3 * t35264 - 0.64311027177104605458e-2 * t31045 + 0.28582678745379824648e-3 * t31049 + 0.18868855373762491241e-2 * t31060 - t37464 + 0.51448821741683684367e-2 * t35273 + 0.34299214494455789578e-2 * t31074 - 0.84046875e-1 * t31081 - 0.5603125e-1 * t31083 - 0.34299214494455789578e-2 * t31095 - 0.85748036236139473944e-2 * t31100;
    (t37471,)
}
