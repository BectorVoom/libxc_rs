//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1034/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1034<F: Float>(t11200: F, t378: F, t3043: F, t3042: F, t993: F, t1071: F, t989: F, t3056: F, t988: F, t1031: F) -> (F, F, F, F, F, F, F, F) {
    let t11201 = t11200 * t378;
    let t11210 = t3043 * t378;
    let t11213 = t3042 * t993;
    let t11214 = t11213 * t378;
    let t11220 = t989 * t1071;
    let t11223 = t988 * t3056;
    let t11224 = t11223 * t378;
    let t11238 = t1031 * t1031;
    let t11239 = F::new(1.0) / t11238;
    (t11201, t11210, t11213, t11214, t11220, t11223, t11224, t11239)
}
