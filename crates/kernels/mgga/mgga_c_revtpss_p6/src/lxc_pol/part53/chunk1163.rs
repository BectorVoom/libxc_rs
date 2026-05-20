//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1163/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1163<F: Float>(t125939: F, t28196: F, t28197: F, t651: F, t7002: F, t7883: F, t27123: F, t8461: F, t27126: F, t1583: F, t7086: F, t27383: F) -> (F, F, F, F, F, F) {
    let t125942 = F::new(4.0) * t28196 * t28197 * t125939;
    let t125945 = t651 * t7883 * t7002;
    let t125948 = F::new(2.0) * t27123 * t8461;
    let t125950 = F::new(2.0) * t27126 * t8461;
    let t125961 = t1583 * t7086;
    let t125962 = t27383 * t125961;
    (t125942, t125945, t125948, t125950, t125961, t125962)
}
