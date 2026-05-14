//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1201/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1201<F: Float>(t35090: F, t35093: F, t35095: F, t35097: F, t35077: F, t35086: F, t37184: F, t37185: F, t37186: F, t37188: F, t37189: F, t35108: F, t35121: F, t35124: F, t35127: F, t35132: F) -> (F, F, F, F, F, F) {
    let t37191 = 0.10298285674687440379e-5 * t35090;
    let t37192 = 0.15716995342493974597e-7 * t35093;
    let t37193 = 0.27012148473991046866e-5 * t35095;
    let t37194 = 0.11594181388521408695e-4 * t35097;
    let t37195 = t37184 - t37185 - t37186 + 0.57970906942607043474e-5 * t35077 - t37188 - t37189 + 0.33460450185846399385e-7 * t35086 + t37191 - t37192 + t37193 + t37194;
    let t37200 = 0.20220636637604418766e-5 * t35108;
    let t37205 = 0.21135226489492151266e-6 * t35121;
    let t37206 = 0.19808908880926767702e-4 * t35124;
    let t37207 = 0.57920616843011475696e-5 * t35127;
    let t37208 = 0.50680539737635041234e-3 * t35132;
    (t37195, t37200, t37205, t37206, t37207, t37208)
}
