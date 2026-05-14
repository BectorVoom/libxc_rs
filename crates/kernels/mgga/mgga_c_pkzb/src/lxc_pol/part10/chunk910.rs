//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 910/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk910<F: Float>(t581: F, t6929: F, t1025: F, t5264: F, t1769: F, t2667: F, t1706: F, t2592: F, t5225: F, t5244: F, t5265: F, t5267: F, t5289: F, t580: F, t6904: F, t6908: F, t6914: F, t6916: F, t6920: F, t6924: F, t6928: F) -> (F, F, F, F) {
    let t6930 = t581 * t6929;
    let t6933 = t5264 * t1025;
    let t6935 = t1769 * t2667;
    let t6937 = 0.12862205435420921092e-2 * t2592 * t6904 - 0.17149607247227894789e-2 * t5244 * t6908 - 35.0 / 108.0 * t5265 + 7.0 / 144.0 * t5267 - t6914 + t1706 * t6916 / 8.0 + t1706 * t6920 / 16.0 - t5225 * t6924 / 4.0 + t6928 - t580 * t6930 / 48.0 - 35.0 / 216.0 * t6933 + 0.80031500487063509014e-2 * t6935 - t5289;
    (t6930, t6933, t6935, t6937)
}
