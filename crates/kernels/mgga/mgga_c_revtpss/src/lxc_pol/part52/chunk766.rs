//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 766/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk766<F: Float>(t25240: F, t2710: F, t826: F, t2482: F, t27: F, t7036: F, t2487: F, t2689: F, t7030: F, t1945: F, t2693: F, t807: F, t2718: F, t64: F, t820: F, t843: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t25242 = t2710 * t25240 * t826;
    let t25243 = 0.90357964994909313586e-5 * t25242;
    let t25245 = t2482 * t7036 * t27;
    let t25246 = t25245 * t2487;
    let t25253 = t2689 * t7030;
    let t25254 = 0.15244095330869239812e-3 * t25253;
    let t25255 = t1945 * t2693;
    let t25256 = t807 * t25255;
    let t25257 = 0.11433071498151929859e-3 * t25256;
    let t25260 = t2718 * t64;
    let t25266 = t820 * t7036 * t843;
    (t25242, t25243, t25245, t25246, t25253, t25254, t25256, t25257, t25260, t25266)
}
