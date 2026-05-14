//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1100/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1100<F: Float>(t21306: F, t21308: F, t21310: F, t21312: F, t21316: F, t21319: F, t21322: F, t21326: F, t21329: F, t21332: F, t21336: F, t20994: F, t20996: F, t20999: F, t21003: F, t21007: F, t21009: F, t21012: F, t21015: F, t21017: F, t21019: F, t21278: F, t21281: F, t21284: F, t21287: F, t21291: F, t21294: F, t21296: F, t21298: F, t21301: F, t21303: F, t22079: F, t22102: F, t22125: F) -> (F,) {
    let t22148 = -0.20833333333333333333e-1 * t21306 - 0.9375e-1 * t21308 - 0.26979166666666666666e-1 * t21310 - 0.33333333333333333334e0 * t21312 + 0.1875e0 * t21316 - 0.41666666666666666666e-1 * t21319 - 0.125e0 * t21322 - 0.9375e-1 * t21326 - 0.625e-1 * t21329 + 0.26979166666666666666e-1 * t21332 + 0.60703125e-1 * t21336;
    let t22151 = t22079 + 0.5e0 * t20994 + 0.20234375e-1 * t20996 + 0.375e0 * t20999 - 0.101171875e-1 * t21003 + 0.101171875e-1 * t21007 + 0.91666666666666666667e0 * t21009 + 0.20234375e-1 * t21012 - 0.20234375e-1 * t21015 + 0.33333333333333333334e0 * t21017 + 0.101171875e-1 * t21019 + t22102 + t22125 - 0.34173611111111111111e0 * t21278 + 0.21583333333333333334e0 * t21281 - 0.125e0 * t21284 - 1.0 * t21287 - 0.1875e0 * t21291 + 0.375e0 * t21294 - 0.33333333333333333334e0 * t21296 + 0.625e-1 * t21298 + 0.125e0 * t21301 - 0.125e0 * t21303 + t22148;
    (t22151,)
}
