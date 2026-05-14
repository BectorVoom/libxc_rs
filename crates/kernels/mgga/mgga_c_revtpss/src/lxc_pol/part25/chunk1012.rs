//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1012/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1012<F: Float>(t2712: F, t64: F, t2710: F, t826: F, t2482: F, t27: F, t7036: F, t2487: F, t2479: F, t7045: F, t2648: F, t7038: F, t2689: F, t7030: F, t1945: F, t2693: F) -> (F, F, F, F, F, F, F, F) {
    let t25240 = t64 * t2712;
    let t25242 = t2710 * t25240 * t826;
    let t25243 = 0.90357964994909313586e-5 * t25242;
    let t25245 = t2482 * t7036 * t27;
    let t25246 = t25245 * t2487;
    let t25248 = t7045 * t2479;
    let t25251 = t7038 * t2648;
    let t25253 = t2689 * t7030;
    let t25254 = 0.15244095330869239812e-3 * t25253;
    let t25255 = t1945 * t2693;
    (t25240, t25243, t25245, t25246, t25248, t25251, t25254, t25255)
}
