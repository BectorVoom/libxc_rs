//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1177/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1177<F: Float>(t125922: F, t32265: F, t32269: F, t125849: F, t552: F, t8590: F, t1405: F, t33959: F, t1448: F, t7933: F, t27123: F, t8461: F) -> (F, F, F, F, F, F) {
    let t125923 = t32265 * t125922;
    let t125925 = t32269 * t125922;
    let t125928 = t125849 * t8590 * t552;
    let t125930 = t33959 * t1405;
    let t125939 = t7933 * t1448;
    let t125948 = F::new(2.0) * t27123 * t8461;
    (t125923, t125925, t125928, t125930, t125939, t125948)
}
