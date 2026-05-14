//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1230/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1230<F: Float>(t3219: F, t91496: F, t23092: F, t46565: F, t1853: F, t6557: F, t8418: F, t22914: F, t25618: F, t38463: F, t5502: F, t26312: F, t8392: F, t26319: F, t26322: F, t26297: F) -> (F, F, F, F, F, F, F, F, F) {
    let t102431 = t91496 * t3219;
    let t102433 = t46565 * t23092;
    let t102436 = t8418 * t6557 * t1853;
    let t102439 = 2.0 / 81.0 * t22914 * t25618;
    let t102442 = t38463 * t5502;
    let t102465 = 4.0 / 27.0 * t8392 * t26312;
    let t102467 = 2.0 / 27.0 * t8392 * t26319;
    let t102469 = 4.0 / 27.0 * t8392 * t26322;
    let t102471 = 4.0 / 81.0 * t8392 * t26297;
    (t102431, t102433, t102436, t102439, t102442, t102465, t102467, t102469, t102471)
}
