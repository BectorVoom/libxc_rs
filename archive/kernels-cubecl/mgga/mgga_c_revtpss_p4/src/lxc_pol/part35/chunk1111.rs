//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1111/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1111<F: Float>(t10690: F, t1945: F, t9646: F, t7030: F, t9789: F, t2453: F, t2783: F, t64: F, t9784: F, t2482: F, t25260: F, t27: F) -> (F, F, F, F, F) {
    let t93007 = t9646 * t1945 * t10690;
    let t93012 = t9789 * t7030;
    let t93015 = t2453 * t2783 * t64;
    let t93020 = t9784 * t7030;
    let t93025 = t2482 * t25260 * t27;
    (t93007, t93012, t93015, t93020, t93025)
}
