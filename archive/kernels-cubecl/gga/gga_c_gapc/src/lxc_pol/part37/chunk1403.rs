//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1403/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1403<F: Float>(t35080: F, t35083: F, t35090: F, t35093: F, t35095: F, t35097: F, t35108: F, t35121: F, t35124: F, t35127: F, t35132: F, t35135: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37188 = F::cast_from(0.40483072916666666668e-3_f64) * t35080;
    let t37189 = F::cast_from(0.18310351929594268994e-5_f64) * t35083;
    let t37191 = F::cast_from(0.10298285674687440379e-5_f64) * t35090;
    let t37192 = F::cast_from(0.15716995342493974597e-7_f64) * t35093;
    let t37193 = F::cast_from(0.27012148473991046866e-5_f64) * t35095;
    let t37194 = F::cast_from(0.11594181388521408695e-4_f64) * t35097;
    let t37200 = F::cast_from(0.20220636637604418766e-5_f64) * t35108;
    let t37205 = F::cast_from(0.21135226489492151266e-6_f64) * t35121;
    let t37206 = F::cast_from(0.19808908880926767702e-4_f64) * t35124;
    let t37207 = F::cast_from(0.57920616843011475696e-5_f64) * t35127;
    let t37208 = F::cast_from(0.50680539737635041234e-3_f64) * t35132;
    let t37210 = F::cast_from(0.43284943850479925795e-3_f64) * t35135;
    (t37188, t37189, t37191, t37192, t37193, t37194, t37200, t37205, t37206, t37207, t37208, t37210)
}
