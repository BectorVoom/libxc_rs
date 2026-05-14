//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 754/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk754<F: Float>(t19240: F, t799: F, t27: F, t89: F, t375: F, t5300: F, t5226: F, t17727: F, t835: F, t446: F, t17732: F, t2857: F, t10758: F, t17749: F, t17753: F, t3281: F) -> (F, F, F, F, F, F, F) {
    let t19241 = t799 * t19240;
    let t19243 = t89 * t27 * t19241;
    let t19246 = t89 * t375 * t5300;
    let t19249 = t89 * t375 * t5226;
    let t19251 = t835 * t17727;
    let t19252 = t446 * t19251;
    let t19254 = t2857 * t17732;
    let t19255 = t446 * t19254;
    let t19257 = t10758 * t17749;
    let t19258 = t446 * t19257;
    let t19260 = t2857 * t17753;
    let t19261 = t3281 * t19260;
    (t19243, t19246, t19249, t19252, t19255, t19258, t19261)
}
