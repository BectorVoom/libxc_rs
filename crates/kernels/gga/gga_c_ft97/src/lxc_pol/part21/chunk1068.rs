//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1068/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1068<F: Float>(t1286: F, t25524: F, t376: F, t25587: F, t5495: F, t26124: F, t5502: F, t8216: F, t22914: F, t25618: F, t38463: F, t26312: F, t8392: F, t26319: F, t26322: F, t26297: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t102364 = 2.0 / 9.0 * t1286 * t376 * t25524;
    let t102366 = 2.0 / 9.0 * t5495 * t25587;
    let t102369 = 2.0 / 9.0 * t1286 * t376 * t26124;
    let t102385 = t8216 * t5502;
    let t102439 = 2.0 / 81.0 * t22914 * t25618;
    let t102442 = t38463 * t5502;
    let t102465 = 4.0 / 27.0 * t8392 * t26312;
    let t102467 = 2.0 / 27.0 * t8392 * t26319;
    let t102469 = 4.0 / 27.0 * t8392 * t26322;
    let t102471 = 4.0 / 81.0 * t8392 * t26297;
    (t102364, t102366, t102369, t102385, t102439, t102442, t102465, t102467, t102469, t102471)
}
