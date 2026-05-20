//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 322/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk322<F: Float>(t934: F, t935: F, t915: F, t902: F, t908: F, t307: F) -> (F, F, F, F, F, F) {
    let t936 = t934 * t935;
    let t938 = F::new(1.0) * t915 * t936;
    let t939 = F::cast_from(0.17123333333333333333e-1_f64) * t902;
    let t941 = -t939 - F::cast_from(0.17123333333333333333e-1_f64) * t908;
    let t944 = t307 * t307;
    let t945 = F::new(1.0) / t944;
    (t936, t938, t939, t941, t944, t945)
}
