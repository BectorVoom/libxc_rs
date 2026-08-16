//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1235/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1235(t29346: f64, t38375: f64, t29348: f64, t29350: f64, t29352: f64, t29354: f64, t29356: f64, t40: f64, t56289: f64, t87: f64, t29365: f64, t29367: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t56299 = 0.1403573615389248977e2_f64 * t29346;
    let t56300 = 0.65061485296689145287e-1_f64 * t38375;
    let t56301 = 0.86748647062252193714e-1_f64 * t29348;
    let t56302 = 0.13012297059337829057e0_f64 * t29350;
    let t56303 = 48.0_f64 * t29352;
    let t56304 = 960.0_f64 * t29354;
    let t56305 = 480.0_f64 * t29356;
    let t56307 = t40 * t56289 * t87;
    let t56308 = 0.14035736153892489771e2_f64 * t29365;
    let t56309 = 0.41015588084031179722e4_f64 * t29367;
    (t56299, t56300, t56301, t56302, t56303, t56304, t56305, t56307, t56308, t56309)
}
