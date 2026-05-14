//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 230/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk230<F: Float>(t650: F, t677: F, t657: F, t668: F, t673: F, t681: F) -> (F, F, F) {
    let t716 = 0.301925e0 * t650;
    let t719 = 0.82785e-1 * t677;
    let t721 = 0.258925e1 * t668 - t716 + 0.905775e0 * t657 + 0.16504875e0 * t673 - t719 + 0.248355e0 * t681;
    (t716, t719, t721)
}
