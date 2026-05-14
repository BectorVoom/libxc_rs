//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 523/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk523<F: Float>(t10657: F, t471: F, t3427: F, t64: F, t2919: F, t871: F, t9664: F, t9666: F, t9674: F, t9676: F) -> (F, F) {
    let t10658 = t10657 * t471;
    let t10660 = 4.0 / 3.0 * t3427 * t64;
    let t10661 = t2919 * t871;
    let t10663 = 7.0 / 256.0 * t9664;
    let t10664 = 21.0 / 8192.0 * t9666;
    let t10665 = 7.0 / 8192.0 * t9674;
    let t10666 = 7.0 / 768.0 * t9676;
    let t10667 = t10658 - t10660 + t10661 / 2.0 - t10663 + t10664 - t10665 + t10666;
    (t10661, t10667)
}
