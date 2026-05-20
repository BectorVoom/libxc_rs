//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1931/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1931<F: Float>(t7063: F, t99271: F, t1568: F, t786: F, t25410: F, t25374: F, t98848: F, t4424: F, t886: F, t4343: F, t605: F, t27383: F, t63164: F) -> (F, F, F, F, F, F, F) {
    let t99373 = t7063 * t99271;
    let t99403 = t786 * t1568;
    let t99404 = t99403 * t25410;
    let t99463 = t98848 * t25374;
    let t99466 = t99403 * t25374;
    let t99512 = t4424 * t886;
    let t99543 = t605 * t4343;
    let t99550 = t27383 * t63164;
    (t99373, t99404, t99463, t99466, t99512, t99543, t99550)
}
