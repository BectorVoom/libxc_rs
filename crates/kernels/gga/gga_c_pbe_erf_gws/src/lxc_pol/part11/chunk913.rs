//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 913/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk913<F: Float>(t1292: F, t13: F, t18515: F, t18648: F, t1276: F, t1285: F, t1291: F, t1274: F, t404: F, t260: F, t262: F, t16578: F, t88: F) -> (F, F, F, F, F, F, F) {
    let t18651 = t1292 * t1292;
    let t18655 = F::cast_from(0.24954977986735470917e5_f64) * t13 / t18648 * t18515 / t18651;
    let t18658 = F::cast_from(36.0_f64) * t1291 * t1276 * t1285;
    let t18664 = t1285 * t1285;
    let t18667 = F::cast_from(6.0_f64) * t1274 * t18664 * t404;
    let t18670 = F::cast_from(1.0_f64) / t260;
    let t18684 = F::cast_from(1.0_f64) / t262;
    let t18708 = t16578 * t88;
    (t18655, t18658, t18664, t18667, t18670, t18684, t18708)
}
