//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2006/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2006<F: Float>(t3063: F, t8521: F, t11200: F, t7143: F, t1035: F, t1983: F, t36870: F, t1096: F, t19482: F, t27668: F, t995: F, t4982: F, t988: F) -> (F, F, F, F, F, F) {
    let t94042 = t3063 * t8521;
    let t94053 = t11200 * t7143;
    let t94063 = t1983 * t36870 * t1035;
    let t94064 = t19482 * t1096;
    let t94080 = t995 * t27668;
    let t94081 = t4982 * t988;
    (t94042, t94053, t94063, t94064, t94080, t94081)
}
