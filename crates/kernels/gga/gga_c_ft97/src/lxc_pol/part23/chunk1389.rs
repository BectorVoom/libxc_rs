//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1389/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1389<F: Float>(t127863: F, t24980: F, t24981: F, t25140: F, t28735: F, t5362: F, t6318: F, t840: F, t856: F, t2862: F, t824: F, t2665: F, t28746: F, t28755: F, t3746: F, t113632: F, t113634: F, t127877: F, t127879: F, t127882: F, t127887: F, t127892: F, t127894: F) -> (F, F, F, F, F) {
    let t127898 = t24980 * t24981 * t25140 * t127863;
    let t127903 = t28735 * t840 * t6318 * t5362 * t856;
    let t127908 = t24980 * t2862 * t6318 * t5362 * t824;
    let t127912 = t28755 * t2665 * t28746 * t3746;
    let t127914 = t127877 - 2.0 / 3.0 * t127879 + 2.0 / 9.0 * t127882 + t113632 + t113634 + t127887 / 2.0 + t127892 - 4.0 / 3.0 * t127894 + t127898 / 6.0 - 3.0 / 8.0 * t127903 - t127908 / 2.0 - 2.0 / 3.0 * t127912;
    (t127898, t127903, t127908, t127912, t127914)
}
