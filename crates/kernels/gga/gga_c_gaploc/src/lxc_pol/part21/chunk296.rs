//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 296/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk296<F: Float>(t155: F, t462: F, t153: F, t1233: F, t1236: F, t1240: F, t1227: F, t145: F, t458: F, t465: F, t1230: F, t1237: F, t1242: F) -> (F, F, F, F, F, F) {
    let pi = F::cast_from(M_PI);
    let t1246 = F::cast_from(1.0_f64) / t462 / t155;
    let t1247 = t153 * t1246;
    let t1248 = t1247 * t1233;
    let t1250 = t1236 * t1240 * pi;
    let t1254 = t1227 * t145 * t458;
    let t1255 = t465 * t1254;
    let t1257 = F::cast_from(63.0_f64) / F::cast_from(256.0_f64) * t1230 - F::cast_from(49.0_f64) / F::cast_from(8192.0_f64) * t1237 * t1242 + F::cast_from(49.0_f64) / F::cast_from(24576.0_f64) * t1248 * t1250 - F::cast_from(21.0_f64) / F::cast_from(256.0_f64) * t1255;
    (t1246, t1247, t1250, t1254, t1255, t1257)
}
