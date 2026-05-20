//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1772/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1772<F: Float>(t225: F, t25286: F, t7048: F, t7071: F, t886: F, t7082: F, t72: F, t686: F) -> (F, F, F, F) {
    let t25287 = t25286 * t225;
    let t25292 = t7071 * t7048 * t886;
    let t25295 = t7082 * t72;
    let t25296 = t25295 * t686;
    (t25287, t25292, t25295, t25296)
}
