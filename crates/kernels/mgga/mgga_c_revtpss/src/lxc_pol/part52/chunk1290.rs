//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1290/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1290<F: Float>(t34332: F, t575: F, t2110: F, t7956: F, t2037: F, t8130: F, t1921: F, t8720: F, t2118: F, t7939: F, t121531: F, t122710: F, t122712: F, t122714: F, t122720: F, t122722: F, t122795: F, t2038: F, t28993: F, t5808: F, t7337: F, t7560: F, t7940: F, t8114: F, t8721: F) -> F {
    let t129127 = t34332 * t575;
    let t129129 = t2110 * t7956;
    let t129130 = t2037 * t8130;
    let t129132 = t8720 * t1921;
    let t129135 = t7939 * t2118;
    let t129136 = t2038 * t28993 + t5808 * t8721 + t7337 * t8114 + t7560 * t7940 + t121531 + t122710 + t122712 + t122714 + t122720 + t122722 + t122795 + t129127 + t129129 + t129130 + t129132 + t129135;
    t129136
}
