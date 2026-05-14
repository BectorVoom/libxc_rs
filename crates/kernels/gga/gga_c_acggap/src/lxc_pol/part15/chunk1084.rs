//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1084/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1084<F: Float>(t31039: F, t32677: F, t35238: F, t35240: F, t35244: F, t37437: F, t37438: F, t37442: F, t37443: F, t37450: F, t37451: F, t39771: F, t39775: F, t39779: F, t39782: F, t39784: F, t39786: F, t39790: F) -> (F,) {
    let t41797 = -0.42874018118069736972e-3 * t39771 - 0.42874018118069736972e-3 * t39775 - t37437 - 0.42874018118069736972e-3 * t39779 - 0.28582678745379824648e-3 * t39782 + 0.64025200389650807211e-1 * t39784 - 0.17149607247227894789e-1 * t39786 + t37438 + t37442 + t37443 - 0.42874018118069736972e-2 * t35238 - 0.25724410870841842184e-1 * t35240 + 0.17149607247227894789e-2 * t35244 - 0.94344276868812456204e-2 * t39790 + t37450 - t37451 + t32677 + 0.80031500487063509014e-2 * t31039;
    (t41797,)
}
