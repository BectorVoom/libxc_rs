//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1177/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1177<F: Float>(t22613: F, t25649: F, t415: F, t22604: F, t25657: F, t1602: F, t5533: F, t1293: F, t929: F, t1294: F, t22833: F, t22568: F, t25718: F, t3099: F, t58: F, t11142: F, t11324: F, t11327: F, t22590: F, t22591: F, t22826: F, t25708: F, t25734: F, t3034: F, t3038: F, t37985: F, t401: F, t409: F, t428: F, t45573: F, t6427: F, t92666: F, t92669: F, t92689: F) -> (F, F, F, F, F, F, F) {
    let t100980 = 0.29693535778629056444e-3 * t22613 * t415 * t25649;
    let t100981 = t25657 * t22604;
    let t100992 = t1602 * t5533;
    let t100999 = t1293 * t929;
    let t101004 = t22833 * t1294;
    let t101013 = t22568 * t25718;
    let t101016 = t58 * t3099;
    let t101021 = 0.44745149797750190322e-9 * t37985 * t92669 * t6427 + 0.93019603785751168e-2 * t100992 * t3038 + 0.93019603785751168e-2 * t25734 * t11324 + 0.46509801892875584e-2 * t25734 * t11327 - 0.27039520901431665706e-3 * t45573 * t409 * t100999 * t428 + 0.93019603785751168e-2 * t101004 * t3038 - 0.38731446812548799881e-3 * t22826 * t11142 + 0.46509801892875584e-1 * t92666 * t3034 + 0.46509801892875584e-1 * t92689 * t3034 + 0.45399899292181069959e-1 * t25708 * t101013 + 0.88910709717637694816e-2 * t22590 * t22591 * t101016 * t401;
    (t100980, t100981, t100992, t101004, t101013, t101016, t101021)
}
