//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta151 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk788;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk789;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk790;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta151<F: Float>(t1082: F, t3059: F, t1086: F, t378: F, t994: F, t1089: F, t3118: F, t1071: F, t359: F, t999: F, t3075: F, t3140: F, t3143: F, t342: F, t3151: F, t335: F, t368: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t3283, t3286, t3287, t3288, t3291, t3292, t3295, t3298) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk788::<F>(t1082, t3059, t1086, t378, t994, t1089, t3118, t1071, t359, t999, t3075, t3140, t3143);
        let t3299 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk789::<F>(t3298, t342);
        let (t3300, t3302) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk790::<F>(t3151, t378, t335, t368);
    (t3283, t3286, t3287, t3288, t3291, t3292, t3295, t3298, t3299, t3300, t3302)
}
