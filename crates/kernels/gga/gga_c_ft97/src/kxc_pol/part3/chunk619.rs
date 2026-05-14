//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 619/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk619<F: Float>(t11401: F, t355: F, t3001: F, t89: F, t3014: F, t376: F, t3196: F, t8392: F, t3190: F, t3115: F, t1882: F, t3257: F, t110: F, t1786: F, t463: F, t488: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11402 = t11401 * t355;
    let t11404 = t89 * t11402 * t3001;
    let t11416 = t89 * t376 * t3014;
    let t11417 = 2.0 / 9.0 * t11416;
    let t11430 = 4.0 / 81.0 * t8392 * t3196;
    let t11436 = 4.0 / 27.0 * t8392 * t3190;
    let t11448 = 2.0 / 27.0 * t8392 * t3115;
    let t11467 = 2.0 / 9.0 * t1882 * t3257;
    let t11468 = t1786 * t110;
    let t11472 = t463 * t488;
    (t11402, t11404, t11416, t11417, t11430, t11436, t11448, t11467, t11468, t11472)
}
