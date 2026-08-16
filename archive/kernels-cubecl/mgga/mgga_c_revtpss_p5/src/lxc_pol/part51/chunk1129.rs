//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1129/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1129<F: Float>(t27375: F, t27383: F, t198: F, t8539: F, t27384: F, t98785: F, t1544: F, t7086: F, t25207: F, t18875: F, t2411: F, t33726: F) -> (F, F, F, F, F, F, F) {
    let t125977 = t27383 * t27375;
    let t125980 = t198 * t8539;
    let t125981 = t98785 * t27384;
    let t125984 = t1544 * t7086;
    let t125985 = t25207 * t125984;
    let t125988 = t27383 * t18875;
    let t125997 = t33726 * t2411;
    (t125977, t125980, t125981, t125984, t125985, t125988, t125997)
}
