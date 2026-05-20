//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 615/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk615<F: Float>(t118: F, t1939: F, t2036: F, t2127: F, t2163: F, t2165: F, t508: F, t569: F, t3: F, param_d: F) -> (F, F, F) {
    let t2167 = -t118 * t2163 - t2127 * t508 + t2165 * t569 - t1939 + t2036;
    let t2168 = t3 * t2167;
    let t2170 = param_d * t2167;
    (t2167, t2168, t2170)
}
