//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1356/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1356<F: Float>(t127029: F, t2665: F, t6317: F, t684: F, t4129: F, t7021: F, t1486: F, t193: F, t2781: F, t113565: F, t113567: F, t113569: F, t113579: F, t127008: F, t127011: F, t127015: F, t127019: F, t127024: F, t127027: F) -> (F, F, F, F) {
    let t127032 = t6317 * t2665 * t127029 * t684;
    let t127034 = t7021 * t4129;
    let t127037 = t1486 * t193 * t2781 * t127034;
    let t127039 = t127008 + t113565 + t113567 - t113569 + t127011 + t127015 / 3.0 - 6.0 * t127019 + 4.0 / 27.0 * t113579 - t127024 - 6.0 * t127027 + t127032 / 6.0 + 2.0 * t127037;
    (t127032, t127034, t127037, t127039)
}
