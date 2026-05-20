//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1164/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1164<F: Float>(t198: F, t8536: F, t1940: F, t2255: F, t8494: F, t27375: F, t27383: F, t8539: F, t27384: F, t98785: F, t1544: F, t7086: F) -> (F, F, F, F, F, F) {
    let t125968 = t198 * t8536;
    let t125976 = t1940 * t8494 * t2255;
    let t125977 = t27383 * t27375;
    let t125980 = t198 * t8539;
    let t125981 = t98785 * t27384;
    let t125984 = t1544 * t7086;
    (t125968, t125976, t125977, t125980, t125981, t125984)
}
