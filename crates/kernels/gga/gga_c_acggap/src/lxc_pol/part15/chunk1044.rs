//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1044/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1044<F: Float>(t38092: F, t7963: F, t9029: F, t7942: F, t8406: F, t10025: F, t157: F, t2146: F, t2152: F, t31965: F, t33080: F, t33093: F, t33097: F, t33100: F, t33104: F, t38153: F, t38157: F, t38165: F, t38176: F, t38685: F, t40675: F, t6068: F, t633: F, t7931: F, t8306: F) -> (F,) {
    let t41164 = t7963 * t38092 * t9029;
    let t41167 = t7942 * t38092 * t8406;
    let t41169 = t38153 + t38157 - 0.13170898365871023197e1 * t33080 + t38165 + 0.4336814094102599731e0 * t2146 * t2152 * t633 * t6068 * t157 - 0.65854491829355115987e0 * t33093 - 0.8673628188205199462e0 * t33097 - t33100 - 0.8673628188205199462e0 * t7931 * t8306 * t38685 - 0.17347256376410398924e1 * t7931 * t8306 * t40675 + t38176 - 0.17347256376410398924e1 * t31965 * t10025 + 0.8673628188205199462e0 * t33104 + 0.17347256376410398924e1 * t41164 - 0.17347256376410398924e1 * t41167;
    (t41169,)
}
