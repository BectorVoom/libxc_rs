//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1203/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1203<F: Float>(t1035: F, t1983: F, t94014: F, t11200: F, t1976: F, t7143: F, t36870: F, t27668: F, t995: F, t25610: F, t25698: F, t378: F) -> (F, F, F, F, F, F, F) {
    let t94016 = t1983 * t94014 * t1035;
    let t94026 = t11200 * t1976;
    let t94053 = t11200 * t7143;
    let t94063 = t1983 * t36870 * t1035;
    let t94080 = t995 * t27668;
    let t94085 = t25610 * t27668;
    let t94121 = t25698 * t378;
    (t94016, t94026, t94053, t94063, t94080, t94085, t94121)
}
