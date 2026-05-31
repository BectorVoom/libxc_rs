//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2253/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2253<F: Float>(t5795: F, t7953: F, t1916: F, t28265: F, t28277: F, t1518: F, t572: F, t670: F, t7741: F, t28280: F, t1459: F, t30191: F) -> (F, F, F, F, F, F) {
    let t109291 = F::cast_from(6.0_f64) * t5795 * t7953;
    let t109293 = F::cast_from(12.0_f64) * t1916 * t28265;
    let t109295 = F::cast_from(12.0_f64) * t1916 * t28277;
    let t109299 = F::cast_from(12.0_f64) * t572 * t670 * t7741 * t1518;
    let t109305 = F::cast_from(6.0_f64) * t1916 * t28280;
    let t109307 = F::cast_from(6.0_f64) * t1459 * t30191;
    (t109291, t109293, t109295, t109299, t109305, t109307)
}
