//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1841/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1841<F: Float>(t25375: F, t27341: F, t25387: F, t1559: F, t886: F, t25392: F, t1955: F, t7057: F) -> (F, F, F, F, F) {
    let t27342 = t25375 * t27341;
    let t27344 = t25387 * t27341;
    let t27349 = t1559 * t886;
    let t27350 = t25392 * t27349;
    let t27353 = t1955 * t7057;
    (t27342, t27344, t27349, t27350, t27353)
}
