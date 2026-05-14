//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 962/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk962<F: Float>(t30280: F, t34082: F, t34092: F, t34095: F, t34100: F, t34102: F, t34107: F, t34131: F, t34133: F, t36950: F, t36961: F, t39026: F, t39029: F, t39031: F, t39035: F, t39039: F, t39041: F, t39043: F) -> (F,) {
    let t39047 = 0.14291339372689912324e-3 * t30280 - t34082 - 0.10718504529517434243e-3 * t39026 - 0.10718504529517434243e-3 * t39029 + 0.94344276868812456204e-2 * t39031 + 0.42874018118069736972e-3 * t39035 - 0.15724046144802076034e-2 * t39039 + 11.0 / 384.0 * t39041 + 11.0 / 1152.0 * t39043 + t34092 - 0.62896184579208304135e-3 * t34095 - t34100 + t34102 + 0.94344276868812456205e-2 * t34107 + t36950 + t34131 - t34133 - t36961;
    (t39047,)
}
