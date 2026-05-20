//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 824/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk824<F: Float>(t27202: F, t789: F, t231: F, t7759: F, t836: F, t7076: F, t27198: F, t867: F, t786: F, t7060: F, t7063: F, t14685: F, t1941: F) -> (F, F, F, F, F, F) {
    let t27203 = t27202 * t789;
    let t27206 = t7759 * t836 * t231;
    let t27207 = t7076 * t27206;
    let t27212 = t27198 * t867;
    let t27213 = t786 * t27212;
    let t27214 = t27213 * t7060;
    let t27216 = t7063 * t27212;
    let t27217 = t27216 * t7060;
    let t27221 = t1941 * t14685;
    (t27203, t27206, t27207, t27214, t27217, t27221)
}
