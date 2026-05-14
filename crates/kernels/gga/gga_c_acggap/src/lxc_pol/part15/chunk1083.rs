//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1083/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1083<F: Float>(t32668: F, t32670: F, t32671: F, t32672: F, t35176: F, t35180: F, t35184: F, t35190: F, t35198: F, t35204: F, t37428: F, t37430: F, t39746: F, t39750: F, t39756: F, t39763: F, t39765: F, t39767: F) -> (F,) {
    let t41784 = t32668 + t32670 - t32671 + t32672 + 0.21437009059034868486e-3 * t39746 + 0.21437009059034868486e-3 * t39750 - 0.83861579438944405516e-3 * t35176 + 0.10718504529517434243e-2 * t39756 + 0.42874018118069736972e-3 * t35180 - 0.83861579438944405517e-3 * t35184 - t37428 + 0.94344276868812456205e-2 * t35190 - t37430 + 0.75475421495049964964e-2 * t35198 - 0.18868855373762491241e-1 * t39763 - 0.85748036236139473944e-3 * t39765 - 0.56606566121287473724e-2 * t39767 - 0.27953859812981468505e-1 * t35204;
    (t41784,)
}
