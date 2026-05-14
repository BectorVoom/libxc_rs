//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 932/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk932<F: Float>(t12051: F, t357: F, t12048: F, t1043: F, t1089: F, t3259: F, t12032: F, t380: F, t11620: F, t378: F, t359: F, t999: F, t11239: F, t3143: F, t342: F, t3154: F) -> (F, F, F, F, F, F, F, F) {
    let t12052 = t12051 * t357;
    let t12053 = t12048 * t12052;
    let t12057 = t3259 * t1043 * t1089;
    let t12066 = t380 * t12032;
    let t12070 = t378 * t11620 * t1089;
    let t12073 = t359 * t3259;
    let t12074 = t12073 * t999;
    let t12077 = t11239 * t3143;
    let t12078 = t342 * t12077;
    let t12079 = t12051 * t3154;
    (t12052, t12053, t12057, t12066, t12070, t12074, t12078, t12079)
}
