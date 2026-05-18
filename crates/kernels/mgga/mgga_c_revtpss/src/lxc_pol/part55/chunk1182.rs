//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1182/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1182<F: Float>(t1405: F, t33959: F, t1448: F, t7933: F, t27123: F, t8461: F, t27126: F, t1583: F, t7086: F, t27383: F, t1544: F, t25207: F) -> (F, F, F, F, F, F, F, F) {
    let t125930 = t33959 * t1405;
    let t125939 = t7933 * t1448;
    let t125948 = F::new(2.0) * t27123 * t8461;
    let t125950 = F::new(2.0) * t27126 * t8461;
    let t125961 = t1583 * t7086;
    let t125962 = t27383 * t125961;
    let t125984 = t1544 * t7086;
    let t125985 = t25207 * t125984;
    (t125930, t125939, t125948, t125950, t125961, t125962, t125984, t125985)
}
