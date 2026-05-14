//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1008/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1008<F: Float>(t2070: F, t41154: F, t25876: F, t26304: F, t25894: F, t2097: F, t22: F, t25937: F, t94696: F, t10115: F, t2099: F, t7493: F, t9292: F, t9646: F, t9648: F, t25875: F) -> (F, F, F, F, F, F, F, F) {
    let t95964 = t2070 * t41154;
    let t96186 = t25876 * t26304;
    let t96187 = t25894 * t96186;
    let t96204 = t25937 * t2097 * t22;
    let t96206 = 0.43639970290213137151e-3 * t94696 * t96204;
    let t96210 = 0.11044544084478153697e-3 * t10115 * t2099;
    let t96218 = 0.17073386770573548589e-1 * t9292 * t7493;
    let t96230 = 0.19637199382202157274e-3 * t9646 * t2097 * t9648;
    let t96236 = t25875 * t96186;
    (t95964, t96187, t96204, t96206, t96210, t96218, t96230, t96236)
}
