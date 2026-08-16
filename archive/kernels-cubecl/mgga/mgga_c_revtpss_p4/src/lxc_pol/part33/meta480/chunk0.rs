//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1755/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1755<F: Float>(t2487: F, t25245: F, t2689: F, t7030: F, t1945: F, t2693: F, t807: F, t2718: F, t64: F) -> (F, F, F, F, F) {
    let t25246 = t25245 * t2487;
    let t25253 = t2689 * t7030;
    let t25254 = F::cast_from(0.15244095330869239812e-3_f64) * t25253;
    let t25255 = t1945 * t2693;
    let t25256 = t807 * t25255;
    let t25260 = t2718 * t64;
    (t25246, t25254, t25255, t25256, t25260)
}
