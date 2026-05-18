//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 241/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk241<F: Float>(t902: F, t307: F, t302: F, t928: F, t310: F) -> (F, F, F, F, F, F, F) {
    let t939 = F::new(0.17123333333333333333e-1) * t902;
    let t944 = t307 * t307;
    let t945 = F::new(1.0) / t944;
    let t946 = t302 * t945;
    let t948 = F::new(0.516475e0) * t902;
    let t951 = F::new(0.104195e0) * t928;
    let t954 = F::new(1.0) / t310;
    (t939, t944, t945, t946, t948, t951, t954)
}
