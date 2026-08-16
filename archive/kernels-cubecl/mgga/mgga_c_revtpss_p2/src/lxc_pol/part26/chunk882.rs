//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 882/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk882<F: Float>(t1071: F, t989: F, t3056: F, t988: F, t378: F, t2258: F, t606: F, t4801: F, t1042: F, t1031: F) -> (F, F, F, F, F, F) {
    let t11220 = t989 * t1071;
    let t11223 = t988 * t3056;
    let t11224 = t11223 * t378;
    let t11231 = t606 * t2258;
    let t11232 = t4801 * t11231;
    let t11233 = t1042 * t11232;
    let t11238 = t1031 * t1031;
    let t11239 = F::cast_from(1.0_f64) / t11238;
    (t11220, t11223, t11224, t11231, t11233, t11239)
}
