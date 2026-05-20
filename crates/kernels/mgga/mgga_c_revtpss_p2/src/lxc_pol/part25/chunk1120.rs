//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1120/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1120<F: Float>(t225: F, t25286: F, t7048: F, t7071: F, t886: F, t7082: F, t72: F, t686: F, t7058: F, t2453: F, t7057: F, t136: F, t1958: F) -> (F, F, F, F, F, F, F) {
    let t25287 = t25286 * t225;
    let t25292 = t7071 * t7048 * t886;
    let t25295 = t7082 * t72;
    let t25296 = t25295 * t686;
    let t25297 = t7058 * t25296;
    let t25299 = t2453 * t7057;
    let t25300 = t1958 * t136;
    (t25287, t25292, t25295, t25296, t25297, t25299, t25300)
}
