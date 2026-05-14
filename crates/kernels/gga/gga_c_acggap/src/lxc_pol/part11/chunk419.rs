//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 419/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk419<F: Float>(t2020: F, t571: F, t2016: F, t580: F, t1143: F, t336: F, t570: F, t167: F, t19: F) -> (F, F, F, F, F) {
    let t2021 = t2020 * t571;
    let t2022 = 7.0 / 288.0 * t2021;
    let t2023 = t2016 * t580;
    let t2024 = 11.0 / 1152.0 * t2023;
    let t2025 = t336 * t1143;
    let t2026 = t570 * t2025;
    let t2028 = t167 * t19;
    (t2022, t2024, t2025, t2026, t2028)
}
