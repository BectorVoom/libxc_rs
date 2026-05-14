//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 974/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk974<F: Float>(t28059: F, t7749: F, t1813: F, t3178: F, t1096: F, t5068: F, t26891: F, t8069: F, t5091: F, t7748: F, t28041: F, t28043: F, t28046: F, t28048: F, t28051: F, t28053: F, t28055: F, t28057: F) -> (F, F, F, F, F, F) {
    let t28060 = t28059 * t7749;
    let t28062 = t3178 * t1813;
    let t28064 = t1096 * t5068;
    let t28066 = t26891 * t8069;
    let t28068 = t7748 * t5091;
    let t28070 = t28041 / 96.0 + t28043 / 8.0 + t28046 / 24.0 - t28048 / 96.0 - t28051 / 16.0 - t28053 / 16.0 + t28055 / 24.0 - t28057 / 9.0 - t28060 / 16.0 + t28062 / 128.0 + t28064 / 128.0 + t28066 / 6.0 - t28068 / 24.0;
    (t28060, t28062, t28064, t28066, t28068, t28070)
}
