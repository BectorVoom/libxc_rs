//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 912/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk912<F: Float>(t22021: F, t3992: F, t2661: F, t550: F, t6861: F, t4003: F, t9934: F, t3989: F, t6856: F, t3957: F, t6884: F, t6850: F, t9744: F) -> (F, F, F, F, F, F, F) {
    let t22022 = t3992 * t22021;
    let t22023 = t2661 * t22022;
    let t22025 = t550 * t6861;
    let t22026 = t22025 * t4003;
    let t22027 = t9934 * t22026;
    let t22028 = t2661 * t22027;
    let t22030 = t3989 * t6856;
    let t22038 = t3957 * t6884;
    let t22044 = t9744 * t6850;
    (t22023, t22025, t22026, t22028, t22030, t22038, t22044)
}
