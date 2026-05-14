//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1417/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1417<F: Float>(t2651: F, t9218: F, t26008: F, t26018: F, t26021: F, t30426: F, t30430: F, t30437: F, t30446: F, t30448: F, t30456: F, t30458: F, t30470: F, t30473: F, t30475: F, t20594: F, t2687: F, t9246: F) -> (F, F) {
    let t34374 = t2651 * t9218;
    let t34387 = 0.34672886960217074253e0 * t34374 + 0.86743646395112941038e-3 * t30426 + 0.38087975358139160776e-1 * t30430 - 0.20803732176130244552e1 * t30437 - t26008 + 0.34930954652346593434e-1 * t30446 + 0.23049072140274290595e1 * t30448 - 0.25426783770825854452e1 * t30456 - 0.12805040077930161442e1 * t30458 - 0.4075278042773769234e0 * t26018 - t26021 + 0.17465477326173296717e-1 * t30470 + 0.34672886960217074253e0 * t30473 + 0.10401866088065122276e1 * t30475;
    let t34390 = t20594 * t2687 * t9246;
    (t34387, t34390)
}
