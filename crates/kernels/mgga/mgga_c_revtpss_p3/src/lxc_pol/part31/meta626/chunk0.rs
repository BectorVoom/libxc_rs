//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2078/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2078<F: Float>(t1096: F, t357: F, t1976: F, t4743: F, t27543: F, t342: F, t4778: F, t8521: F, t1078: F, t42859: F, t1983: F, t3143: F) -> (F, F, F, F, F, F) {
    let t99566 = t357 * t1096;
    let t99629 = t4743 * t1976;
    let t99666 = t342 * t27543;
    let t99675 = t4778 * t8521;
    let t99682 = t42859 * t1078;
    let t99684 = t1983 * t99682 * t3143;
    (t99566, t99629, t99666, t99675, t99682, t99684)
}
