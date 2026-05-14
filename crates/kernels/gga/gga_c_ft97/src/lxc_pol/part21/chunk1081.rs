//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1081/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1081<F: Float>(t22870: F, t6414: F, t22914: F, t25564: F, t1286: F, t25848: F, t376: F, t25529: F, t25534: F, t27435: F, t5: F, t1360: F, t7954: F, t165: F, t7763: F, t23405: F, t27417: F) -> (F, F, F, F, F, F, F, F, F) {
    let t103975 = t6414 * t22870;
    let t104006 = 2.0 / 3.0 * t22914 * t25564;
    let t104016 = t1286 * t376 * t25848 / 9.0;
    let t104025 = 2.0 / 9.0 * t1286 * t376 * t25529;
    let t104031 = 2.0 / 9.0 * t1286 * t376 * t25534;
    let t104095 = t5 * t27435;
    let t104150 = t7954 * t1360;
    let t104151 = t165 * t7763;
    let t104157 = 2.0 / 27.0 * t23405 * t27417;
    (t103975, t104006, t104016, t104025, t104031, t104095, t104150, t104151, t104157)
}
