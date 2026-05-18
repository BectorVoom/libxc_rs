//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1235/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1235<F: Float>(t29346: F, t38375: F, t29348: F, t29350: F, t29352: F, t29354: F, t29356: F, t40: F, t56289: F, t87: F, t29365: F, t29367: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t56299 = F::new(0.1403573615389248977e2) * t29346;
    let t56300 = F::new(0.65061485296689145287e-1) * t38375;
    let t56301 = F::new(0.86748647062252193714e-1) * t29348;
    let t56302 = F::new(0.13012297059337829057e0) * t29350;
    let t56303 = F::new(48.0) * t29352;
    let t56304 = F::new(960.0) * t29354;
    let t56305 = F::new(480.0) * t29356;
    let t56307 = t40 * t56289 * t87;
    let t56308 = F::new(0.14035736153892489771e2) * t29365;
    let t56309 = F::new(0.41015588084031179722e4) * t29367;
    (t56299, t56300, t56301, t56302, t56303, t56304, t56305, t56307, t56308, t56309)
}
