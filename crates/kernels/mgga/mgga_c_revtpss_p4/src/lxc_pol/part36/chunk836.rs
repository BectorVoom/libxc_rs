//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 836/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk836<F: Float>(t234: F, t2735: F, t2699: F, t798: F, t159: F, t853: F, t216: F, t2729: F, t794: F, t10111: F, t22: F, t870: F) -> (F, F, F, F, F) {
    let t10886 = t2735 * t234;
    let t10890 = t2699 * t798;
    let t10899 = t159 * t853;
    let t10900 = t216 * t10899;
    let t10905 = t794 * t2729;
    let t10939 = F::cast_from(0.19637199382202157274e-3_f64) * t10111 * t870 * t22;
    (t10886, t10890, t10900, t10905, t10939)
}
