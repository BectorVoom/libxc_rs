//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2163/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2163<F: Float>(t1364: F, t30074: F, t786: F, t1882: F, t543: F, t5774: F, t30020: F, t686: F, t72: F, t25895: F, t1398: F, t6918: F) -> (F, F, F, F, F) {
    let t108175 = t786 * t30074 * t1364;
    let t108178 = t5774 * t1882 * t543;
    let t108187 = t30020 * t72 * t686;
    let t108188 = t25895 * t108187;
    let t108206 = t6918 * t1398 * t543;
    (t108175, t108178, t108187, t108188, t108206)
}
