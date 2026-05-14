//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1009/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1009<F: Float>(t31037: F, t31039: F, t35211: F, t35213: F, t35228: F, t35231: F, t35249: F, t35251: F, t37446: F, t37447: F, t37449: F, t39771: F, t39775: F, t39779: F, t39782: F, t39784: F, t39786: F, t39790: F) -> (F,) {
    let t39793 = -0.21437009059034868486e-3 * t39771 - 0.21437009059034868486e-3 * t39775 - t35211 - 0.21437009059034868486e-3 * t39779 - 0.14291339372689912324e-3 * t39782 + 0.32012600194825403606e-1 * t39784 - 0.85748036236139473945e-2 * t39786 + t35213 + t35228 + t35231 - t37446 - t37447 + t37449 - 0.47172138434406228102e-2 * t39790 + t35249 - t35251 + t31037 + 0.40015750243531754508e-2 * t31039;
    (t39793,)
}
