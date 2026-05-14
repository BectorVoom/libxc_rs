//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 292/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk292<F: Float>(t155: F, t462: F, t153: F, t1233: F, t1236: F, t1240: F, t1227: F, t145: F, t458: F, t465: F, t1230: F, t1237: F, t1242: F, t157: F, t470: F, t471: F, t64: F, t90: F) -> (F, F, F, F, F) {
    let t1246 = 1.0 / t462 / t155;
    let t1247 = t153 * t1246;
    let t1248 = t1247 * t1233;
    let t1250 = t1236 * t1240 * M_PI;
    let t1254 = t1227 * t145 * t458;
    let t1255 = t465 * t1254;
    let t1257 = 63.0 / 256.0 * t1230 - 49.0 / 8192.0 * t1237 * t1242 + 49.0 / 24576.0 * t1248 * t1250 - 21.0 / 256.0 * t1255;
    let t1265 = t1257 * t471 - 4.0 / 3.0 * t470 * t64 + 7.0 / 96.0 * t1230 - 7.0 / 288.0 * t1255 + 4.0 / 3.0 * t157 * t90;
    (t1246, t1247, t1250, t1254, t1265)
}
