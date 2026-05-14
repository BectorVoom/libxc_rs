//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1100/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1100<F: Float>(t35835: F, t37739: F, t37741: F, t40196: F, t40200: F, t40204: F, t40208: F, t40212: F, t40218: F, t40220: F, t40222: F, t40226: F, t40230: F, t40234: F, t40237: F, t40241: F, t40243: F, t40246: F) -> (F,) {
    let t42006 = 0.62896184579208304137e-2 * t40196 + 0.62896184579208304137e-2 * t40200 + 0.41930789719472202758e-2 * t40204 - 0.21437009059034868486e-3 * t40208 - 0.64311027177104605458e-2 * t40212 + 0.18868855373762491241e-2 * t40218 - 0.31448092289604152068e-2 * t40220 - 0.51448821741683684368e-2 * t35835 + t37739 - 0.12862205435420921092e-2 * t40222 + 0.62896184579208304138e-3 * t40226 + 0.62896184579208304138e-3 * t40230 + 0.31448092289604152069e-3 * t40234 + t37741 + 0.42874018118069736972e-3 * t40237 - 0.94344276868812456207e-3 * t40241 - 0.34299214494455789578e-1 * t40243 + 0.21437009059034868486e-2 * t40246;
    (t42006,)
}
