//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 857/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk857<F: Float>(t1876: F, t23249: F, t11490: F, t11863: F, t22959: F, t1911: F, t5691: F, t8557: F, t1901: F, t23199: F, t23203: F, t23208: F, t23212: F, t23216: F, t23220: F, t23224: F, t23227: F, t23229: F, t23232: F, t23236: F, t23239: F, t23241: F, t23246: F, t446: F) -> (F, F, F, F, F, F) {
    let t23250 = t23249 * t1876;
    let t23251 = t11490 * t23250;
    let t23254 = t11863 * t22959;
    let t23257 = t5691 * t1911;
    let t23258 = t8557 * t23257;
    let t23261 = -2.0 / 9.0 * t23199 + 2.0 / 3.0 * t446 * t23203 + t446 * t23208 / 3.0 + 2.0 / 3.0 * t446 * t23212 + 4.0 / 3.0 * t446 * t23216 + 4.0 / 3.0 * t446 * t23220 + 2.0 / 3.0 * t446 * t23224 - 4.0 / 9.0 * t23227 - 2.0 / 9.0 * t23229 + t1901 * t23232 / 9.0 + 2.0 / 27.0 * t1901 * t23236 - 2.0 / 27.0 * t23239 + 2.0 / 9.0 * t1901 * t23241 + 2.0 / 9.0 * t1901 * t23246 - 4.0 / 3.0 * t1901 * t23251 - 4.0 / 9.0 * t1901 * t23254 - 2.0 / 9.0 * t1901 * t23258;
    (t23250, t23251, t23254, t23257, t23258, t23261)
}
