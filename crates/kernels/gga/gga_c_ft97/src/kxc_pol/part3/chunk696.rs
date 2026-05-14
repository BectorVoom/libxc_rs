//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 696/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk696<F: Float>(t11987: F, t16219: F, t16169: F, t3194: F, t3193: F, t11567: F, t11578: F, t11593: F, t11610: F, t11612: F, t11632: F, t11821: F, t11826: F, t16200: F, t16205: F, t16210: F, t16213: F, t16216: F, t1901: F, t446: F, t8233: F) -> (F,) {
    let t16220 = t11987 * t16219;
    let t16223 = t3194 * t16169;
    let t16224 = t3193 * t16223;
    let t16227 = -t11567 + 8.0 / 27.0 * t11578 - 2.0 * t446 * t16200 + 4.0 / 3.0 * t446 * t16205 - 4.0 / 81.0 * t8233 + 2.0 / 3.0 * t446 * t16210 + t16213 / 9.0 + t11610 - t11612 - t11632 - t446 * t16216 / 3.0 + t11821 - t11826 - 10.0 / 81.0 * t1901 * t16220 - 8.0 / 27.0 * t11593 * t16224;
    (t16227,)
}
