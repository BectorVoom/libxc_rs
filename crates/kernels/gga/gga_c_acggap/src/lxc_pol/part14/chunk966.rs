//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 966/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk966<F: Float>(t1851: F, t7614: F, t30308: F, t30310: F, t30314: F, t30319: F, t34189: F, t34204: F, t34215: F, t34218: F, t34222: F, t39080: F, t39082: F, t39086: F, t39088: F, t39092: F, t39094: F, t39098: F, t39100: F) -> (F,) {
    let t39107 = t7614 * t1851;
    let t39109 = -0.85748036236139473944e-3 * t39080 + 0.85748036236139473944e-3 * t39082 - 0.62896184579208304134e-2 * t34189 - 0.80031500487063509016e-2 * t34204 - 0.85748036236139473944e-3 * t39086 + 0.85748036236139473944e-3 * t39088 - 0.7145669686344956162e-4 * t39092 - 0.16006300097412701803e-1 * t39094 + 0.47172138434406228102e-2 * t39098 - 0.94344276868812456204e-3 * t39100 - 77.0 / 576.0 * t30308 - 77.0 / 1728.0 * t30310 - 0.38203125e-2 * t30314 + 0.80031500487063509016e-2 * t30319 - 0.62896184579208304136e-3 * t34215 - t34218 - t34222 - 0.32012600194825403606e-1 * t39107;
    (t39109,)
}
