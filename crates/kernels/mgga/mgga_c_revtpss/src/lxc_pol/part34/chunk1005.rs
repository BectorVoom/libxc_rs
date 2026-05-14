//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1005/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1005<F: Float>(t1968: F, t3080: F, t1973: F, t3201: F, t25516: F, t3114: F, t3057: F, t7143: F, t1035: F, t8515: F, t1983: F, t378: F, t7150: F, t8521: F, t995: F, t1976: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t25538 = t1968 * t3080 / 432.0;
    let t25560 = 0.95275595817932748827e-4 * t1973 * t3201;
    let t25580 = t3114 * t25516;
    let t25591 = t3057 * t7143;
    let t25604 = t8515 * t1035;
    let t25605 = t1983 * t25604;
    let t25610 = t7150 * t378;
    let t25611 = t25610 * t8521;
    let t25629 = t995 * t8521;
    let t25651 = t3057 * t1976;
    (t25538, t25560, t25580, t25591, t25604, t25605, t25610, t25611, t25629, t25651)
}
