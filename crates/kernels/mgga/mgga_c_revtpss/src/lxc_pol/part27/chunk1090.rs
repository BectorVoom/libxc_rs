//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1090/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1090<F: Float>(t2487: F, t25245: F, t2479: F, t7045: F, t2648: F, t7038: F, t2689: F, t7030: F, t1945: F, t2693: F, t807: F, t2756: F) -> (F, F, F, F, F, F, F) {
    let t25246 = t25245 * t2487;
    let t25248 = t7045 * t2479;
    let t25251 = t7038 * t2648;
    let t25253 = t2689 * t7030;
    let t25254 = F::new(0.15244095330869239812e-3) * t25253;
    let t25255 = t1945 * t2693;
    let t25256 = t807 * t25255;
    let t25257 = F::new(0.11433071498151929859e-3) * t25256;
    let t25258 = t7038 * t2756;
    (t25246, t25248, t25251, t25254, t25255, t25257, t25258)
}
