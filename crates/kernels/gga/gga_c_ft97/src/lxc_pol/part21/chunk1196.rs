//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1196/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1196<F: Float>(t1882: F, t29971: F, t29911: F, t8392: F, t29907: F, t29982: F, t29936: F, t376: F, t89: F, t102836: F, t102838: F, t103108: F, t11810: F, t16120: F, t1825: F, t1871: F, t1901: F, t1909: F, t23294: F, t26171: F, t26172: F, t26176: F, t26356: F, t29839: F, t3266: F, t3271: F, t4458: F, t446: F, t5717: F, t60901: F, t7750: F, t91629: F, t971: F) -> (F,) {
    let t117277 = t1882 * t29971;
    let t117279 = t8392 * t29911;
    let t117281 = t8392 * t29907;
    let t117292 = t1882 * t29982;
    let t117295 = t89 * t376 * t29936;
    let t117316 = -2.0 / 9.0 * t117277 - 2.0 / 27.0 * t117279 - 2.0 / 27.0 * t117281 - 4.0 / 3.0 * t1901 * t11810 * t5717 * t16120 + t102836 + t102838 + 4.0 / 27.0 * t91629 - 2.0 / 3.0 * t446 * t1871 * t1825 * t29839 - 4.0 / 9.0 * t117292 - t117295 / 9.0 - 2.0 / 9.0 * t1901 * t1909 * t23294 * t4458 - 4.0 * t1901 * t26171 * t26356 * t3266 - 4.0 / 3.0 * t1901 * t11810 * t103108 * t3271 - 4.0 / 3.0 * t1901 * t60901 * t26176 - 4.0 * t1901 * t7750 * t971 * t26172;
    (t117316,)
}
