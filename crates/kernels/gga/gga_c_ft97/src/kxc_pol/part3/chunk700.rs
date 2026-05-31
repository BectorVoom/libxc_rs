//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 700/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk700<F: Float>(t3343: F, t376: F, t89: F, t11402: F, t3330: F, t7773: F, t998: F, t3409: F, t375: F, t3379: F, t549: F, t135: F, t3347: F) -> (F, F, F, F, F, F, F, F) {
    let t12356 = t89 * t376 * t3343;
    let t12357 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t12356;
    let t12359 = t89 * t11402 * t3330;
    let t12362 = t89 * t7773 * t998;
    let t12365 = t89 * t375 * t3409;
    let t12366 = t12365 / F::cast_from(9.0_f64);
    let t12367 = t549 * t3379;
    let t12374 = t3347 * t135;
    (t12356, t12357, t12359, t12362, t12365, t12366, t12367, t12374)
}
