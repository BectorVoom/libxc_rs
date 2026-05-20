//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1914/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1914<F: Float>(t102928: F, t25375: F, t1957: F, t28425: F, t25372: F, t98809: F, t25386: F, t95822: F, t98815: F, t95537: F, t25310: F, t28360: F) -> (F, F, F, F, F, F) {
    let t102930 = F::cast_from(0.28912093960683998208e-1_f64) * t25375 * t102928;
    let t102931 = t1957 * t28425;
    let t102934 = F::cast_from(0.28912093960683998208e-1_f64) * t25372 * t102931 * t98809;
    let t102937 = F::cast_from(0.51405703062096148812e-1_f64) * t25386 * t102931 * t98809;
    let t102939 = F::cast_from(0.28912093960683998208e-1_f64) * t95822 * t98815;
    let t102941 = F::cast_from(0.51405703062096148812e-1_f64) * t95537 * t98815;
    let t102943 = F::cast_from(0.14456046980341999104e-1_f64) * t25310 * t28360;
    (t102930, t102934, t102937, t102939, t102941, t102943)
}
