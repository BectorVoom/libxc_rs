//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 995/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk995<F: Float>(t159: F, t3181: F, t2851: F, t631: F, t45: F, t1071: F, t3057: F, t992: F, t338: F, t378: F, t3056: F, t988: F, t1031: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11142 = t159 * t3181;
    let t11144 = 1.0 / t2851 / t631;
    let t11149 = t2851 * t45;
    let t11150 = 1.0 / t11149;
    let t11187 = t3057 * t1071;
    let t11198 = t992 * t992;
    let t11199 = 1.0 / t11198;
    let t11200 = t338 * t11199;
    let t11201 = t11200 * t378;
    let t11223 = t988 * t3056;
    let t11224 = t11223 * t378;
    let t11238 = t1031 * t1031;
    let t11239 = 1.0 / t11238;
    (t11142, t11144, t11150, t11187, t11200, t11201, t11223, t11224, t11239)
}
