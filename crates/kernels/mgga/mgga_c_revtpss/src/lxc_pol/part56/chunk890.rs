//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 890/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk890<F: Float>(t196: F, t197: F, t8237: F, t2035: F, t7935: F, t8764: F, t13272: F, t8736: F, t7937: F, t2163: F, t7741: F, t651: F, t7586: F, t7742: F, t1937: F, t29427: F) -> (F, F, F, F, F, F, F, F, F) {
    let t34399 = t8237 * t196 * t197;
    let t34400 = t34399 * t2035;
    let t34401 = t8764 * t7935;
    let t34402 = t13272 * t8736;
    let t34424 = t8764 * t7937;
    let t34428 = t2163 * t7741;
    let t34429 = t651 * t34428;
    let t34434 = t7586 * t7742;
    let t34444 = t29427 * t1937;
    (t34399, t34400, t34401, t34402, t34424, t34428, t34429, t34434, t34444)
}
